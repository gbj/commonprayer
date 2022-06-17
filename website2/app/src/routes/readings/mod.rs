pub mod eucharist;
pub mod office;

use calendar::Date;
use language::Language;
use leptos2::*;
use liturgy::Version;

use crate::{utils::time::today, views::bible_version_select_options};

pub struct ReadingsView {
    pub locale: String,
    pub date: Date,
    pub version: Version,
    pub path: String,
}

#[derive(Params)]
pub struct ReadingsViewQuery {
    date: Option<String>,
    version: Option<Version>,
}

#[async_trait]
impl Loader for ReadingsView {
    type Params = ();
    type Query = ReadingsViewQuery;

    async fn loader(
        locale: &str,
        path: &str,
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
        Some(Self {
            locale: locale.to_string(),
            date,
            version,
            path: path.to_string(),
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
            include_str!("readings.css").into(),
            include_str!("../../styles/reading-link-table.css").into(),
            include_str!("../../styles/toggle-fieldset.css").into(),
            include_str!("../../styles/toggle-links.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <>
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
                    </div>

                    {nested_view.unwrap_or_default()}
                </main>
            </>
        }
    }
}
