pub mod eucharist;
pub mod holy_day;
pub mod office;
pub mod reading_loader;

use calendar::Date;
use language::Language;
use leptos2::*;
use liturgy::Version;

use crate::{utils::time::today, views::bible_version_select_options};

use super::settings::{DisplaySettings, Settings};

pub struct ReadingsView {
    pub locale: String,
    pub date: Date,
    pub version: Version,
    pub path: String,
    pub display_settings: DisplaySettings,
}

#[derive(Params)]
pub struct ReadingsViewQuery {
    date: Option<String>,
    version: Option<Version>,
}

#[async_trait(?Send)]
impl Loader for ReadingsView {
    type Params = ();
    type Query = ReadingsViewQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let date = query
            .date
            .as_ref()
            .and_then(|date| Date::parse_from_str(date, "%Y-%m-%d").ok())
            .unwrap_or_else(today);
        let version = query
            .version
            .filter(Version::is_bible_translation)
            .unwrap_or(Version::NRSV);

        let display_settings = Settings::display(&req).await;

        Some(Self {
            locale: locale.to_string(),
            date,
            version,
            path: req.path().to_string(),
            display_settings,
        })
    }
}

impl View for ReadingsView {
    fn title(&self) -> String {
        format!(
            "{}: {} – {}",
            t!("toc.readings"),
            self.date
                .to_localized_name(Language::from_locale(&self.locale)),
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../../styles/document.css").into(),
            include_str!("../../styles/reading-link-table.css").into(),
            include_str!("../../styles/toggle-fieldset.css").into(),
            include_str!("../../styles/toggle-links.css").into(),
            include_str!("readings.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <div class={self.display_settings.to_class()}>
                <header><h1>{t!("toc.readings")}</h1></header>
                <main>
                    <form>
                        <fieldset class="horizontal">
                            <label class="stacked">
                                {t!("date")}
                                <input
                                    type="date"
                                    name="date"
                                    value={&self.date.to_padded_string()}
                                    onchange="this.form.submit()"
                                />
                            </label>
                            <label class="stacked">
                                {t!("settings.bible_version")}
                                <select name="version" onchange="this.form.submit()">
                                    {bible_version_select_options(&self.locale, self.version)}
                                </select>
                            </label>
                        </fieldset>
                        <noscript><input type="submit" value={t!("lectionary.go")}/></noscript>
                    </form>
                    <div class="toggle-links">
                        <a href={format!("/{}/readings/office/?date={}&version={}", self.locale, self.date, self.version)}
                            class:current={self.path.contains("/office")}
                        >
                            {t!("toc.daily_office")}
                        </a>
                        <a href={format!("/{}/readings/eucharist/?date={}&version={}", self.locale, self.date, self.version)}
                            class:current={self.path.contains("/eucharist")}
                        >
                            {t!("toc.holy_eucharist")}
                        </a>
                        <a href={format!("/{}/readings/holy-day/?date={}&version={}", self.locale, self.date, self.version)}
                            class:current={self.path.contains("/holy-day")}
                        >
                            {t!("toc.holy_days")}
                        </a>
                    </div>

                    {nested_view.unwrap_or_default()}
                </main>
            </div>
        }
    }
}
