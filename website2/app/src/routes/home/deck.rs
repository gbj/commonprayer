use std::pin::Pin;

use super::Settings;
use api::summary::{DailySummary, EucharisticLectionarySummary};
use calendar::{
    lff2018::LFF_BIOS, Calendar, Date, Feast, LiturgicalDay, Season, Weekday, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use futures::{future::join_all, join, Future};
use language::Language;
use leptos2::{
    http::{HeaderMap, Response},
    *,
};
use library::{summary, CommonPrayer};
use liturgy::{Document, SlugPath};

use crate::{
    api::document_action::FavoriteId,
    routes::settings::{CommonLiturgySettings, GeneralSettings},
    UserInfo,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TodaysDeck(Vec<Card>);

impl IntoIterator for TodaysDeck {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TodaysDeck {
    pub async fn create(
        req: &Arc<dyn Request>,
        locale: &str,
        lff: bool,
        date: Date,
        hour: u32,
    ) -> Self {
        let template = Deck::from_req(req).await;
        let language = Language::from_locale(locale);
        let evening = hour >= 16;
        let calendar = if lff {
            LFF2018_CALENDAR
        } else {
            BCP1979_CALENDAR
        };
        let day = calendar.liturgical_day(date, evening);

        let general_settings = Settings::general(req).await;
        let liturgy_settings =
            CommonLiturgySettings::from_req(req, general_settings.liturgy_version).await;

        let cards = join_all(template.into_iter().map(move |card| {
            let req = req.clone();
            match card {
                CardType::DailySummary => {
                    let name = summary::localize_day_name(&day, &day.observed, &calendar, language);
                    let season = calendar.season(&day);
                    let day = day.clone();
                    let liturgy_settings = liturgy_settings.clone();
                    Box::pin(async move {
                        Some(Card::DailySummary {
                            name,
                            season,
                            evening,
                            lff,
                            day,
                            summary: Box::new(CommonPrayer::daily_office_summary(&date, language)),
                            liturgy_settings,
                        })
                    }) as Pin<Box<dyn Future<Output = Option<Card>>>>
                }
                CardType::SundaySummary => {
                    let sunday_after = if date.weekday() == Weekday::Sun {
                        date
                    } else {
                        date.sunday_before().add_days(7)
                    };
                    let sunday_after = calendar.liturgical_day(sunday_after, false);
                    let season = calendar.season(&sunday_after);
                    let name = summary::localize_day_name(
                        &sunday_after,
                        &sunday_after.observed,
                        &calendar,
                        language,
                    );
                    let summary = CommonPrayer::eucharistic_lectionary_summary_with_day(
                        sunday_after.clone(),
                        language,
                    );
                    let liturgy_settings = liturgy_settings.clone();
                    Box::pin(async move {
                        Some(Card::SundaySummary {
                            name,
                            season,
                            day: sunday_after,
                            summary: Box::new(summary),
                            liturgy_settings,
                        })
                    })
                }
                CardType::HolyDayPreview => {
                    let maybe_day = day.holy_days.get(0).and_then(|feast| {
                        let name = calendar.feast_name(*feast, language);
                        let date = day.date;
                        let bio = LFF_BIOS
                            .iter()
                            .find(|(s_feast, _)| *s_feast == *feast)
                            .map(|(_, bio)| bio.to_string());
                        match (name, bio) {
                            (Some(name), Some(bio)) => Some(Card::HolyDayPreview {
                                id: *feast,
                                name,
                                date,
                                bio,
                            }),
                            _ => None,
                        }
                    });
                    Box::pin(async move { maybe_day })
                }
                CardType::Favorites => Box::pin(async move {
                    let favorites = Favorites::from_req(&req).await;
                    Some(Card::Favorites(favorites))
                }),
                CardType::Bookmarks => Box::pin(async move {
                    let bookmarks = Bookmarks::from_req(&req).await;
                    Some(Card::Bookmarks(bookmarks))
                }),
            }
        }))
        .await;

        Self(cards.into_iter().flatten().collect())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Card {
    DailySummary {
        name: String,
        season: Season,
        evening: bool,
        lff: bool,
        day: LiturgicalDay,
        summary: Box<DailySummary>,
        liturgy_settings: CommonLiturgySettings,
    },
    SundaySummary {
        name: String,
        season: Season,
        day: LiturgicalDay,
        summary: Box<EucharisticLectionarySummary>,
        liturgy_settings: CommonLiturgySettings,
    },
    HolyDayPreview {
        id: Feast,
        name: String,
        date: Date,
        bio: String,
    },
    Favorites(Favorites),
    Bookmarks(Bookmarks),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Deck {
    cards: Vec<CardType>,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            cards: vec![
                CardType::DailySummary,
                CardType::HolyDayPreview,
                CardType::SundaySummary,
                CardType::Favorites,
                CardType::Bookmarks,
            ],
        }
    }
}

impl IntoIterator for Deck {
    type Item = CardType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Deck {
    pub async fn from_req(req: &Arc<dyn Request>) -> Self {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!(
                "SELECT deck from home_page where user_id = $1",
                uid.to_string()
            )
            .fetch_optional(req.db())
            .await
            {
                Ok(Some(res)) => match serde_json::from_value(res.deck) {
                    Ok(deck) => deck,
                    Err(e) => {
                        eprintln!("[Deck::from_req] {e}");
                        Self::default()
                    }
                },
                Ok(None) => Self::default(),
                Err(e) => {
                    eprintln!("[Deck::from_req] {e}");
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CardType {
    DailySummary,
    SundaySummary,
    HolyDayPreview,
    Favorites,
    Bookmarks,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HolyDayPreview {
    id: Feast,
    name: String,
    date: Date,
    bio: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Favorites(pub Vec<(FavoriteId, Document)>);

impl Favorites {
    pub async fn from_req(req: &Arc<dyn Request>) -> Self {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!(
                "SELECT id, content from favorites where user_id = $1",
                uid.to_string()
            )
            .fetch_all(req.db())
            .await
            {
                Ok(res) => Self(
                    res.into_iter()
                        .filter_map(|row| match serde_json::from_value(row.content) {
                            Ok(content) => Some((FavoriteId(row.id), content)),
                            Err(e) => {
                                eprintln!("[Favorites::from_req] {e}");
                                None
                            }
                        })
                        .collect(),
                ),
                Err(e) => {
                    eprintln!("[Favorites::from_req] {e}");
                    Self::default()
                }
            }
        } else {
            req.headers()
                .cookies()
                .filter_map(|cookie| match cookie {
                    Ok(cookie) => Some(cookie),
                    Err(e) => {
                        eprintln!("invalid cookie: {:#?}", e);
                        None
                    }
                })
                .find(|cookie| cookie.name() == "favorites")
                .and_then(|cookie| {
                    urlencoding::decode(cookie.value())
                        .ok()
                        .and_then(|value| serde_json::from_str(&value).ok())
                })
                .unwrap_or_default()
        }
    }

    pub async fn add(
        req: &Arc<dyn Request>,
        headers: &mut HeaderMap,
        favorite: Document,
        date_created: Date,
    ) -> Result<FavoriteId, ()> {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            sqlx::query!(
                "INSERT INTO favorites (user_id, content, date_created) VALUES ($1, $2, $3) RETURNING id;",
                uid.to_string(),
                serde_json::to_value(favorite).unwrap(),
                date_created.as_chrono()
            )
            .fetch_one(req.db())
            .await
            .map(|resp| FavoriteId(resp.id))
            .map_err(|e| {
                eprintln!("[Favorites::add] {}", e);
            })
        } else {
            let mut favorites = Favorites::from_req(req).await;
            let next_id = favorites
                .0
                .iter()
                .map(|(id, _)| id)
                .max()
                .map(|id| FavoriteId(id.0 + 1))
                .unwrap_or_default();
            favorites.0.push((next_id, favorite));

            let json = serde_json::to_string(&favorites).unwrap_or_default();
            let favorites_cookie = cookie::Cookie::build("favorites", urlencoding::encode(&json))
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(cookie::time::Duration::days(365_000))
                .finish();
            headers.insert(
                "Set-Cookie",
                http::HeaderValue::from_str(&favorites_cookie.to_string()).unwrap(),
            );

            Ok(next_id)
        }
    }

    pub async fn remove(
        req: &Arc<dyn Request>,
        headers: &mut HeaderMap,
        id: FavoriteId,
    ) -> Result<(), ()> {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            sqlx::query!(
                "DELETE FROM favorites WHERE id = $1 and user_id = $2;",
                id.0,
                uid.to_string(),
            )
            .execute(req.db())
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!("[Favorites::remove] {}", e);
            })
        } else {
            let mut favorites = Favorites::from_req(&req).await;
            favorites.0.retain(|(s_id, _)| s_id != &id);

            let json = serde_json::to_string(&favorites).unwrap_or_default();
            let favorites_cookie = cookie::Cookie::build("favorites", urlencoding::encode(&json))
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(cookie::time::Duration::days(365_000))
                .finish();
            headers.insert(
                "Set-Cookie",
                http::HeaderValue::from_str(&favorites_cookie.to_string()).unwrap(),
            );

            Ok(())
        }
    }
}

impl IntoIterator for Favorites {
    type Item = (FavoriteId, Document);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Bookmarks(Vec<Bookmark>);

impl Bookmarks {
    pub async fn from_req(req: &Arc<dyn Request>) -> Self {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query_as!(
                Bookmark,
                "SELECT url, label, preview from bookmarks where user_id = $1",
                uid.to_string()
            )
            .fetch_all(req.db())
            .await
            {
                Ok(res) => Self(res),
                Err(e) => {
                    eprintln!("[Favorites::from_req] {e}");
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }
}

impl IntoIterator for Bookmarks {
    type Item = Bookmark;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bookmark {
    url: String,
    label: String,
    preview: String,
}
