use crate::{
    views::{readings::title_view, DocumentView},
    WebView,
};
use api::summary::{DailySummary, EucharisticLectionarySummary, EucharisticObservanceSummary};
use calendar::{LiturgicalDay, Season, BCP1979_CALENDAR, LFF2018_CALENDAR};
use language::Language;
use leptos2::*;
use library::{summary, CommonPrayer};
use liturgy::Slug;

use crate::utils::time::{current_hour, today};

use super::{
    readings::office::reading_links,
    settings::{GeneralSettings, Settings},
};

pub struct HomePage {
    locale: String,
    language: Language,
    day: LiturgicalDay,
    season: Season,
    localized_day_name: String,
    daily_summary: DailySummary,
    eucharistic_summary: EucharisticLectionarySummary,
    general_settings: GeneralSettings,
}

#[async_trait(?Send)]
impl Loader for HomePage {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let general_settings = GeneralSettings::get_all(&req);

        let calendar = if general_settings.use_lff {
            LFF2018_CALENDAR
        } else {
            BCP1979_CALENDAR
        };
        let date = today();
        let hour = current_hour();
        let day = calendar.liturgical_day(date, hour >= 16);

        let language = Language::from_locale(locale);
        let localized_day_name =
            summary::localize_day_name(&day, &day.observed, &calendar, language);
        let season = calendar.season(&day);

        Some(Self {
            locale: locale.to_string(),
            language,
            day,
            season,
            localized_day_name,
            daily_summary: CommonPrayer::daily_office_summary(&date, language),
            eucharistic_summary: CommonPrayer::eucharistic_lectionary_summary(&date, language),
            general_settings,
        })
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
                        {self.daily_office_card(self.day.evening, true)}
                    </section>
                </main>
            </div>
        }
    }
}

impl HomePage {
    fn daily_office_card(&self, evening: bool, lff: bool) -> Node {
        let observed = &if evening {
            &self.daily_summary.evening
        } else {
            &self.daily_summary.morning
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
                    let url = format!("/{}/holy-day/{:#?}", &self.locale, feast);
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
            &self.daily_summary.morning.observed.daily_office_readings,
            &self.daily_summary.evening.observed.daily_office_readings,
            &self.daily_summary.morning.observed.daily_office_psalms,
            &self.daily_summary.evening.observed.daily_office_psalms,
            format!(
                "/{}/readings/office/?date={}&version={:?}",
                self.locale,
                self.day.date.to_padded_string(),
                self.general_settings.bible_version
            ),
        );

        view! {
            <article class={format!("card {:#?}", self.season)}>
                <header>
                    <h1>{t!("toc.daily_office")}</h1>
                    <h2>{title_view(&self.locale, &self.day.observed, &self.localized_day_name)}</h2>
                    <h3>{self.day.date.to_localized_name(self.language)}</h3>
                </header>
                <main>
                    {black_letter_days}
                    <ul class="office-links">
                        <li>{self.office_link(Slug::MorningPrayer)}</li>
                        <li>{self.office_link(Slug::NoondayPrayer)}</li>
                        <li>{self.office_link(Slug::EveningPrayer)}</li>
                        <li>{self.office_link(Slug::Compline)}</li>
                    </ul>
                    {reading_links.view(&self.locale)}
                </main>
            </article>
        }
    }

    fn office_link(&self, slug: Slug) -> Node {
        let href = format!(
            "/{}/document/office/{}/{:?}/?date={}",
            self.locale,
            slug,
            self.general_settings.liturgy_version,
            self.day.date.to_padded_string()
        );
        view! {
            <a class="badge" href={href}>{t!(&format!("slug.{}", slug))}</a>
        }
    }
}
