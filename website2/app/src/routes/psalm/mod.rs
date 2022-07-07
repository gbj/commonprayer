use crate::{routes::document::views::psalm, Icon};
use language::Language;
use leptos2::*;
use liturgy::{Psalm, Version};
use psalter::{bcp1979::BCP1979_PSALTER, loc::LOC_PSALTER};

pub struct PsalmView<'a> {
    locale: String,
    number: u8,
    psalm: &'a Psalm,
    version: Version,
}

#[derive(Params)]
pub struct PsalmViewQuery {
    number: Option<u8>,
    version: Option<Version>,
}

#[async_trait(?Send)]
impl<'a> Loader for PsalmView<'a> {
    type Params = ();
    type Query = PsalmViewQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let language = Language::from_locale(&locale);
        let version = query.version.unwrap_or(if language == Language::Es {
            Version::LibroDeOracionComun
        } else {
            Version::BCP1979
        });

        let number = query
            .number
            .map(|number| {
                if number > 150 {
                    150
                } else if number < 1 {
                    1
                } else {
                    number
                }
            })
            .unwrap_or(1);

        let psalm = if version == Version::LibroDeOracionComun {
            LOC_PSALTER.psalm_by_number(number)
        } else {
            BCP1979_PSALTER.psalm_by_number(number)
        };

        psalm.map(|psalm| Self {
            locale: locale.to_string(),
            number,
            psalm,
            version,
        })
    }
}

impl<'a> View for PsalmView<'a> {
    fn title(&self) -> String {
        format!(
            "{} – {}",
            t!("daily_readings.psalm", number = &self.number.to_string()),
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../../styles/document.css").into(),
            include_str!("../../styles/toggle-fieldset.css").into(),
            include_str!("psalm.css").into(),
        ]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        let (header, main) = psalm(self.psalm);

        view! {
            <div>
                <header>
                    <a href={format!("/{}/psalm?number={}&version={:?}", self.locale, self.number - 1, self.version)}>
                        <img src={Icon::Left} alt={t!("psalm.prev")}/>
                    </a>
                    <h1>{t!("daily_readings.psalm", number = &self.number.to_string())}</h1>
                    <a href={format!("/{}/psalm?number={}&version={:?}", self.locale, self.number + 1, self.version)}>
                        <img src={Icon::Right} alt={t!("psalm.next")}/>
                    </a>
                </header>
                <main>
                    <form>
                        <label class="stacked">
                            {t!("psalm.psalm")}
                            <input type="number" min="1" max="150" name="number" value={self.number} onchange="this.form.submit()"/>
                        </label>
                        <fieldset class="toggle">
                            <input id="bcp" type="radio" name="version" value="BCP1979" checked={self.version != Version::LibroDeOracionComun} onchange="this.form.submit()"/>
                            <label for="bcp">{t!("psalm.en")}</label>
                            <input id="loc" type="radio" name="version" value="LibroDeOracionComun" checked={self.version == Version::LibroDeOracionComun} onchange="this.form.submit()"/>
                            <label for="loc">{t!("psalm.es")}</label>
                        </fieldset>
                        <noscript><input type="submit" value={t!("lectionary.go")}/></noscript>
                    </form>
                    <article class="document">
                        {header}
                        {main}
                    </article>
                </main>
            </div>
        }
    }

    fn error_boundary(error: RouterError) -> Body
    where
        Self: Sized,
    {
        view! {
            <div>
                <style>
                    {include_str!("../../styles/toggle-fieldset.css")}
                    {include_str!("psalm.css")}
                </style>
                <header><h1>{t!("psalm.psalm")}</h1></header>
                <main class="error">
                    <form>
                        <label class="stacked">
                            {t!("psalm.psalm")}
                            <input type="number" min="1" max="150" name="number" value={1} onchange="this.form.submit()"/>
                        </label>
                        <fieldset class="toggle">
                            <input id="bcp" type="radio" name="version" value="BCP1979" checked={true} onchange="this.form.submit()"/>
                            <label for="bcp">{t!("psalm.en")}</label>
                            <input id="loc" type="radio" name="version" value="LibroDeOracionComun" onchange="this.form.submit()"/>
                            <label for="loc">{t!("psalm.es")}</label>
                        </fieldset>
                        <noscript><input type="submit" value={t!("lectionary.go")}/></noscript>
                    </form>
                    <p class="error-message">{t!("psalm.error")}</p>
                </main>
            </div>
        }
    }
}
