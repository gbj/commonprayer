use crate::utils::encode_uri;
use crate::views::{menu, Header};
use leptos2::{view::View, *};

#[derive(Debug)]
pub struct Index {
    locale: String,
    path: String,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            path: String::new(),
        }
    }
}

#[async_trait]
impl Loader for Index {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        path: &str,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            locale: locale.to_string(),
            path: path.to_string(),
        })
    }
}

impl View for Index {
    fn title(&self) -> String {
        t!("common_prayer")
    }

    fn meta(&self) -> MetaTags {
        vec![
            ("charset".to_string(), "UTF-8".to_string()),
            (
                "viewport".to_string(),
                "width=device-width, initial-scale=1.0".to_string(),
            ),
        ]
    }

    fn links(&self) -> Vec<Node> {
        view! {
            <>
                <link rel="preload" href="/static/fonts/Sabon_Roman.woff2" _as="font" type="font/woff2" crossorigin />
                <link rel="preload" href="/static/fonts/Sabon_Bold.woff2" _as="font" type="font/woff2" crossorigin/>
                <link rel="preload" href="/static/fonts/Sabon_Italic.woff2" _as="font" type="font/woff2" crossorigin/>
                <link rel="preload" href="/static/fonts/Sabon_BoldItalic.woff2" _as="font" type="font/woff2" crossorigin/>
            </>
        }
    }

    fn styles(&self) -> view::Styles {
        vec![
            include_str!("vars.css").into(),
            include_str!("general.css").into(),
            include_str!("header.css").into(),
            include_str!("menu.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Vec<Node> {
        let menu = menu(&self.locale, &self.path);

        let mut index_content = match nested_view {
            Some(view) => vec![menu, view],
            None => {
                view! {
                    <>
                        {menu}
                        <header><h1>{t!("common_prayer")}</h1></header>
                        <main>
                            <h1>{t!("common_prayer")}</h1>
                            {table_of_contents(&self.locale)}
                        </main>
                    </>
                }
            }
        };

        index_content.extend(body_scripts());
        index_content.push(view! {
			<script defer data-domain="commonprayeronline.org" src="https://plausible.io/js/plausible.js"></script>
		});
        index_content
    }

    fn error_boundary(error: RouterError) -> Body {
        view! {
            <>
                <style>{include_str!("404.css")}</style>
                <header><h1>{t!("common_prayer")}</h1></header>
                <main>
                    <h1>{t!("page_404.uh_oh")}</h1>
                    <p>{t!("page_404.explanation")}</p>
                    <section class="links">
                        <a class="email-link" href=format!("mailto:greg@venite.app?subject=(Common Prayer) Error&body={}", encode_uri(&format!("{:#?}", error)))>
                            {t!("page_404.email_greg")}
                        </a>
                        <a class="toc-link" href="/">{t!("page_404.button_text")}</a>
                    </section>
                </main>
            </>
        }
    }
}

fn table_of_contents(locale: &str) -> Node {
    view! {
        <ul class="toc-menu">
            <li>
                {t!("toc.calendar_full")}
                <ul>
                    <li>
                        {make_link(locale, "calendar", "bcp_1979", false)}
                    </li>
                    <li>
                        {make_link(locale, "calendar/lff2018", "lff_2018", false)}
                    </li>
                    <li>
                        {make_link(locale, "readings", "toc.daily_readings", false)}
                    </li>
                </ul>
            </li>
            <li>
                {make_link(locale, "document/office", "toc.daily_office", false)}
                <ul>
                    <li>{make_link(locale, "daily-office", "toc.pray_daily_office", false)}</li>
                    <li>
                        {t!("toc.morning_prayer")}
                        <ul>
                            <li>
                                {make_link(locale, "document/office/morning-prayer/RiteI", "rite_i", true)}
                            </li>
                            <li>
                                {make_link(locale, "document/office/morning-prayer/RiteII", "rite_ii", false)}
                            </li>
                        </ul>
                    </li>
                    <li>
                        {make_link(locale, "document/office/noonday-prayer", "toc.noonday_prayer", false)}
                    </li>
                    <li>
                        {t!("toc.evening_prayer")}
                        <ul>
                            <li>
                                {make_link(locale, "document/office/evening-prayer/RiteI", "rite_i", true)}
                            </li>
                            <li>
                                {make_link(locale, "document/office/evening-prayer/RiteII", "rite_ii", false)}
                            </li>
                        </ul>
                    </li>
                    <li>
                        {make_link(locale, "document/office/compline", "toc.compline", false)}
                    </li>
                    <li>
                        {make_link(locale, "canticle-table", "toc.canticle_table", false)}
                    </li>
                </ul>
            </li>
            <li>
                {make_link(locale, "document/great-litany", "toc.great_litany", false)}
            </li>
            <li>
                {make_link(locale, "document/collects", "collects", false)}
                <ul>
                    <li>
                        {make_link(locale, "document/collects/RiteI", "traditional", false)}
                    </li>
                    <li>
                        {make_link(locale, "document/collects/RiteII", "contemporary", false)}
                    </li>
                </ul>
            </li>
            <li>
                {make_link(locale, "document/proper-liturgies", "toc.proper_liturgies", true)}
            </li>
            <li>
                {make_link(locale, "document/baptism", "toc.holy_baptism", false)}
            </li>
            <li>
                {make_link(locale, "document/eucharist", "toc.holy_eucharist", false)}
            </li>
            <li>
                {t!("toc.pastoral_offices")}
                <ul>
                    <li>
                        {make_link(locale, "document/pastoral-offices/marriage", "toc.marriage", false)}
                    </li>
                    <li>
                        {make_link(locale, "document/pastoral-offices/burial", "toc.burial", false)}
                    </li>
                </ul>
            </li>
            <li>
                {make_link(locale, "document/occasional-services", "toc.occasional_services", false)}
            </li>
            <li>
                {make_link(locale, "document/episcopal-services", "toc.episcopal_services", true)}
            </li>
            <li>
                {make_link(locale, "psalter", "psalter.full_title", false)}
            </li>
            <li>
                {make_link(locale, "document/prayers-and-thanksgivings", "toc.prayers_and_thanksgivings", false)}
            </li>
            <li>
                {make_link(locale, "catechism", "toc.catechism_full", true)}
            </li>
            <li>
                {make_link(locale, "historical-documents", "toc.historical_documents", true)}
            </li>
            <li>
                {make_link(locale, "lectionary", "lectionary.the_lectionary", false)}
            </li>
            <li>
                {make_link(locale, "readings", "toc.daily_office_lectionary", false)}
            </li>
        </ul>
    }
}

fn make_link(
    locale: &str,
    path: &'static str,
    label_i18n_slug: &'static str,
    work_in_progress: bool,
) -> Node {
    if work_in_progress {
        view! {
            <span>
                {t!(label_i18n_slug)}
                " "
                <em>{t!("toc.work_in_progress")}</em>
            </span>
        }
    } else {
        view! {
            <a href={format!("/{}/{}", locale, path)}>
                {t!(label_i18n_slug)}
            </a>
        }
    }
}
