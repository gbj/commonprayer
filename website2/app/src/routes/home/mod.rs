use crate::{routes::document::views::DocumentView, Icon, UserInfo, WebView};
use api::summary::{
    DailySummary, EucharisticLectionarySummary, EucharisticObservanceSummary, TrackedReadings,
};
use calendar::{Date, Feast, LiturgicalDay, Season, BCP1979_CALENDAR, LFF2018_CALENDAR};
use language::Language;
use lectionary::RCLTrack;
use leptos2::{
    http::{HeaderMap, HeaderValue, Response},
    *,
};
use library::{summary, CommonPrayer, Library};
use liturgy::{Document, Slug, Version};

use crate::utils::time::{current_hour, today};

use super::{
    readings::{office::reading_links, views::title_view},
    settings::{GeneralSettings, Settings},
};

mod deck;
use crate::api::document_action::*;
use deck::*;
pub struct HomePage {
    locale: String,
    deck: TodaysDeck,
    general_settings: GeneralSettings,
}

#[derive(Params)]
pub struct HomePageAction {
    pub action: DocumentActionType,
    pub payload: String,
    pub date_created: Option<String>,
}

#[async_trait(?Send)]
impl Loader for HomePage {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        let general_settings = Settings::general(&req).await;

        let deck = TodaysDeck::create(
            &req,
            locale,
            general_settings.use_lff,
            today(),
            current_hour(),
        )
        .await;

        Some(Self {
            locale: locale.to_string(),
            deck,
            general_settings,
        })
    }

    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        if let Some(body) = req.body() {
            match body.as_form_data::<HomePageAction>() {
                Err(e) => {
                    eprintln!("[Home::action] error parsing FormData: {}", e);
                    ActionResponse::from_error(e)
                }
                Ok(data) => match data.action {
                    DocumentActionType::MarkFavorite => {
                        let date_created = data
                            .date_created
                            .map(|date| {
                                Date::parse_from_str(&date, "%Y-%m-%d").unwrap_or_else(|e| {
                                    eprintln!("[Home::action] error parsing date: {e}");
                                    today()
                                })
                            })
                            .unwrap_or_else(today);

                        match serde_json::from_str::<Document>(&data.payload) {
                            Err(e) => {
                                eprintln!("[Home::action] error parsing Document payload: {}", e);
                                ActionResponse::from_error(e)
                            }
                            Ok(favorite) => {
                                let mut headers = HeaderMap::new();

                                match Favorites::add(&req, &mut headers, favorite, date_created)
                                    .await
                                {
                                    Ok(id) => ActionResponse::from_json_with_headers(id, headers),
                                    Err(e) => {
                                        eprintln!("[Home::action] error while adding favorite");
                                        let mut res = Response::builder()
                                            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                                            .body(())
                                            .expect("couldn't build Response");
                                        *res.headers_mut() = headers;
                                        ActionResponse::from_response(res)
                                    }
                                }
                            }
                        }
                    }
                    DocumentActionType::RemoveFavorite => {
                        let id = match serde_json::from_str::<FavoriteId>(&data.payload) {
                            Ok(id) => id,
                            Err(e) => {
                                eprintln!(
                                    "[Home::action - DocumentActionType::RemoveFavorite] {e}"
                                );
                                return ActionResponse::from_error(e);
                            }
                        };

                        let mut headers = HeaderMap::new();
                        headers.insert("Location", format!("/{}", locale).parse().unwrap());

                        match Favorites::remove(&req, &mut headers, id).await {
                            Ok(_) => {
                                let mut res = Response::builder()
                                    .status(http::StatusCode::SEE_OTHER)
                                    .body(())
                                    .expect("couldn't build Response");
                                *res.headers_mut() = headers;

                                ActionResponse::from_response(res)
                            }
                            Err(_) => {
                                eprintln!("[Home::action] error while removing favorite");

                                let mut res = Response::builder()
                                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(())
                                    .expect("couldn't build Response");
                                *res.headers_mut() = headers;

                                ActionResponse::from_response(res)
                            }
                        }
                    }
                },
            }
        } else {
            ActionResponse::None
        }
    }
}

impl View for HomePage {
    fn title(&self) -> String {
        t!("common_prayer")
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../../styles/cards.css").into(),
            include_str!("../../styles/reading-link-table.css").into(),
            include_str!("home.css").into(),
            include_str!("../../styles/document.css").into(),
        ]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header><h1>{t!("common_prayer")}</h1></header>
                <main>
                    <section class="cards">
                        {self.deck.into_iter().map(|card| card.view(&self.locale, &self.general_settings)).collect::<Vec<_>>()}
                    </section>
                </main>
            </div>
        }
    }
}

impl Card {
    fn view(self, locale: &str, settings: &GeneralSettings) -> Node {
        match self {
            Card::DailySummary {
                name,
                season,
                evening,
                lff,
                day,
                summary,
            } => daily_office_card(locale, settings, day, *summary, evening, lff, name, season),
            Card::SundaySummary {
                name,
                season,
                day,
                summary,
            } => sunday_card(locale, name, season, day, settings, *summary),
            Card::HolyDayPreview {
                id,
                name,
                date,
                bio,
            } => holy_day_card(locale, id, name, date, bio),
            Card::Favorites(favorites) => favorites_view(favorites, locale),
            Card::Bookmarks(_) => {
                Node::default()
                //view! {<article class="card">/* <main>"TODO bookmarks"</main> */</article> }
            }
        }
    }
}

fn daily_office_card(
    locale: &str,
    settings: &GeneralSettings,
    day: LiturgicalDay,
    summary: DailySummary,
    evening: bool,
    lff: bool,
    name: String,
    season: Season,
) -> Node {
    let language = Language::from_locale(locale);

    let observed = &if evening {
        &summary.evening
    } else {
        &summary.morning
    }
    .observed;
    let black_letter_days = &if lff {
        &observed.lff_black_letter_days
    } else {
        &observed.bcp_black_letter_days
    };
    let black_letter_days = if black_letter_days.is_empty() {
        Node::default()
    } else {
        let days = black_letter_days
            .iter()
            .map(|(feast, name)| {
                let url = format!("/{}/holy-day/{:#?}", locale, feast);
                view! {
                    <li>
                        <a href={url}>{name}</a>
                    </li>
                }
            })
            .collect::<Vec<_>>();
        view! { <ul class="black-letter-days">{days}</ul>}
    };

    let reading_links = reading_links(
        &summary.morning.observed.daily_office_readings,
        &summary.evening.observed.daily_office_readings,
        &summary.morning.observed.daily_office_psalms,
        &summary.evening.observed.daily_office_psalms,
        format!(
            "/{}/readings/office/?date={}&version={:?}",
            locale,
            day.date.to_padded_string(),
            settings.bible_version
        ),
    );

    view! {
        <article class={format!("card {:?}", season)}>
            <header>
                <div>
                    <h1>{t!("toc.daily_office")}</h1>
                    <h2>{title_view(locale, &day.observed, &name)}</h2>
                    <h3>{day.date.to_localized_name(language)}</h3>
                </div>
            </header>
            <main>
                {black_letter_days}
                <ul class="office-links">
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::MorningPrayer)}</li>
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::NoondayPrayer)}</li>
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::EveningPrayer)}</li>
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::Compline)}</li>
                </ul>
                {reading_links.view(&locale)}
            </main>
        </article>
    }
}

fn office_link(locale: &str, version: Version, date: Date, slug: Slug) -> Node {
    let href = format!(
        "/{}/document/office/{}/{:?}/?date={}",
        locale,
        slug,
        version,
        date.to_padded_string()
    );
    view! {
        <a class="badge" href={href}>{t!(&format!("slug.{}", slug))}</a>
    }
}

fn sunday_card(
    locale: &str,
    name: String,
    season: Season,
    day: LiturgicalDay,
    settings: &GeneralSettings,
    summary: EucharisticLectionarySummary,
) -> Node {
    fn reading_link(
        locale: &str,
        date: Date,
        version: Version,
        citations: &[(Option<RCLTrack>, String)],
    ) -> Node {
        let date = date.to_padded_string();
        let links = citations
            .iter()
            .map(|(track, citation)| {
                let track = match track {
                    Some(RCLTrack::One) => "&track=one",
                    Some(RCLTrack::Two) => "&track=two",
                    None => "",
                };
                let href = format!(
                    "/{locale}/readings/eucharist?date={date}{track}&version={version}#{citation}"
                );
                view! {
                    <a href={href}>{citation}</a>
                }
            })
            .intersperse_with(|| text(format!(" {} ", t!("daily_readings.or"))))
            .collect::<Vec<_>>();
        view! {
            <li>{links}</li>
        }
    }

    let language = Language::from_locale(locale);
    let observed = summary.observed;

    let first_lesson = match &observed.tracked_readings {
        TrackedReadings::Any(readings) => readings
            .first_lesson
            .iter()
            .map(|citation| (None, citation.clone()))
            .collect::<Vec<_>>(),
        TrackedReadings::Tracked {
            track_one,
            track_two,
        } => track_one
            .first_lesson
            .iter()
            .map(|citation| (Some(RCLTrack::One), citation.clone()))
            .chain(
                track_two
                    .first_lesson
                    .iter()
                    .map(|citation| (Some(RCLTrack::Two), citation.clone())),
            )
            .collect(),
    };

    let psalm = match &observed.tracked_readings {
        TrackedReadings::Any(readings) => readings
            .psalm
            .iter()
            .filter_map(|doc| doc.as_citation().map(|citation| (None, citation)))
            .collect::<Vec<_>>(),
        TrackedReadings::Tracked {
            track_one,
            track_two,
        } => track_one
            .psalm
            .iter()
            .filter_map(|doc| {
                doc.as_citation()
                    .map(|citation| (Some(RCLTrack::One), citation))
            })
            .chain(track_two.psalm.iter().filter_map(|doc| {
                doc.as_citation()
                    .map(|citation| (Some(RCLTrack::Two), citation))
            }))
            .collect(),
    };

    let epistle = observed
        .epistle
        .iter()
        .map(|citation| (None, citation.clone()))
        .collect::<Vec<_>>();
    let gospel = observed
        .gospel
        .iter()
        .map(|citation| (None, citation.clone()))
        .collect::<Vec<_>>();

    view! {
        <article class={format!("card {:?}", season)}>
            <header>
                <div>
                    <h1>{t!("home.sunday")}</h1>
                    <h2>{title_view(&locale, &day.observed, &name)}</h2>
                    <h3>{day.date.to_localized_name(language)}</h3>
                </div>
            </header>
            <main>
                <ul class="office-links">
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::MorningPrayer)}</li>
                    <li>
                        <a class="badge" href={format!(
                            "/{}/document/eucharist/eucharist/{:?}/?date={}",
                            locale,
                            settings.liturgy_version,
                            day.date.to_padded_string()
                        )}>
                            {t!("slug.Eucharist")}
                        </a>
                    </li>
                    <li>{office_link(locale, settings.liturgy_version, day.date, Slug::EveningPrayer)}</li>
                </ul>

                <ul class="reading-links">
                    {reading_link(locale, day.date, settings.bible_version, &first_lesson)}
                    {reading_link(locale, day.date, settings.bible_version, &psalm)}
                    {reading_link(locale, day.date, settings.bible_version, &epistle)}
                    {reading_link(locale, day.date, settings.bible_version, &gospel)}
                </ul>
            </main>
        </article>
    }
}

fn holy_day_card(locale: &str, id: Feast, name: String, date: Date, bio: String) -> Node {
    let language = Language::from_locale(locale);
    let read_more_link = bio.len() > 450;
    let truncated_bio = if read_more_link {
        bio.chars()
            .take(450)
            .chain(std::iter::once('…'))
            .collect::<String>()
    } else {
        bio
    }
    .split("\n\n")
    .map(|para| view! {<p>{para}</p>})
    .collect::<Vec<_>>();
    let href = format!("/{}/readings/holy-day?id={:?}", locale, id);
    let bio_href = format!("{}#bio", href);

    view! {
        <article class="card">
            <header>
                <div>
                    <h1><em>{t!("lff")}</em></h1>
                    <h2><a href={href}>{name}</a></h2>
                    <h3>{date.to_localized_name_without_year(language)}</h3>
                </div>
            </header>
            <main>
                {truncated_bio}
                {read_more_link.then(|| view! {
                    <a href={bio_href}>{t!("home.read_more")}</a>
                })}
            </main>
        </article>
    }
}

fn favorites_view(favorites: Favorites, locale: &str) -> Node {
    let cards = favorites
        .into_iter()
        .map(|(id, favorite)| {
            let doc_view = DocumentView {
                path: vec![],
                doc: &favorite,
                url: ""
            };
            view! {
                <article class="card">
                    <header>
                        <div class="buttons">
                            <form method="POST">
                                <input type="hidden" name="action" value={DocumentActionType::RemoveFavorite.to_string()}/>
                                <input type="hidden" name="payload" value={serde_json::to_string(&id).expect("couldn't serialize FavoriteId")}/>
                                <button type="submit">
                                    <img src={Icon::Close.to_string()} alt={t!("document_action.remove_favorite")}/>
                                </button>
                            </form>
                        </div>
                    </header>
                    <main>
                        {doc_view.view(locale)}
                    </main>
                </article>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section class="favorites">
            {(!cards.is_empty()).then(|| view! { <h2>{t!("home.favorites")}</h2>})}
            <div class="cards">
                {cards}
            </div>
        </section>
    }
}
