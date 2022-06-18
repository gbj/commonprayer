use std::sync::Arc;

use leptos2::*;

pub struct ContentsView {
    locale: String,
}

#[async_trait(?Send)]
impl Loader for ContentsView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            locale: locale.to_string(),
        })
    }
}

impl View for ContentsView {
    fn title(&self) -> String {
        format!("{} – {}", t!("toc.table_of_contents"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header><h1>{t!("toc.table_of_contents")}</h1></header>
                <main>
                    <ul class="toc-menu">
                        <li>
                            {t!("toc.calendar_full")}
                            <ul>
                                <li>
                                    {self.make_link("calendar", "bcp_1979", false)}
                                </li>
                                <li>
                                    {self.make_link("calendar/lff2018", "lff_2018", false)}
                                </li>
                                <li>
                                    {self.make_link("readings", "toc.daily_readings", false)}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {self.make_link("document/office", "toc.daily_office", false)}
                            <ul>
                                <li>{self.make_link("daily-office", "toc.pray_daily_office", false)}</li>
                                <li>
                                    {t!("toc.morning_prayer")}
                                    <ul>
                                        <li>
                                            {self.make_link("document/office/morning-prayer/RiteI", "rite_i", true)}
                                        </li>
                                        <li>
                                            {self.make_link("document/office/morning-prayer/RiteII", "rite_ii", false)}
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    {self.make_link("document/office/noonday-prayer", "toc.noonday_prayer", false)}
                                </li>
                                <li>
                                    {t!("toc.evening_prayer")}
                                    <ul>
                                        <li>
                                            {self.make_link("document/office/evening-prayer/RiteI", "rite_i", true)}
                                        </li>
                                        <li>
                                            {self.make_link("document/office/evening-prayer/RiteII", "rite_ii", false)}
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    {self.make_link("document/office/compline", "toc.compline", false)}
                                </li>
                                <li>
                                    {self.make_link("canticle-table", "toc.canticle_table", false)}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {self.make_link("document/great-litany", "toc.great_litany", false)}
                        </li>
                        <li>
                            {self.make_link("document/collects", "collects", false)}
                            <ul>
                                <li>
                                    {self.make_link("document/collects/RiteI", "traditional", false)}
                                </li>
                                <li>
                                    {self.make_link("document/collects/RiteII", "contemporary", false)}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {self.make_link("document/proper-liturgies", "toc.proper_liturgies", true)}
                        </li>
                        <li>
                            {self.make_link("document/baptism", "toc.holy_baptism", false)}
                        </li>
                        <li>
                            {self.make_link("document/eucharist", "toc.holy_eucharist", false)}
                        </li>
                        <li>
                            {t!("toc.pastoral_offices")}
                            <ul>
                                <li>
                                    {self.make_link("document/pastoral-offices/marriage", "toc.marriage", false)}
                                </li>
                                <li>
                                    {self.make_link("document/pastoral-offices/burial", "toc.burial", false)}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {self.make_link("document/occasional-services", "toc.occasional_services", false)}
                        </li>
                        <li>
                            {self.make_link("document/episcopal-services", "toc.episcopal_services", true)}
                        </li>
                        <li>
                            {self.make_link("psalter", "psalter.full_title", false)}
                        </li>
                        <li>
                            {self.make_link("document/prayers-and-thanksgivings", "toc.prayers_and_thanksgivings", false)}
                        </li>
                        <li>
                            {self.make_link("catechism", "toc.catechism_full", true)}
                        </li>
                        <li>
                            {self.make_link("historical-documents", "toc.historical_documents", true)}
                        </li>
                        <li>
                            {self.make_link("lectionary", "lectionary.the_lectionary", false)}
                        </li>
                        <li>
                            {self.make_link("readings", "toc.daily_office_lectionary", false)}
                        </li>
                    </ul>
                </main>
            </div>
        }
    }
}

impl ContentsView {
    fn make_link(
        &self,
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
                <a href={format!("/{}/{}", self.locale, path)}>
                    {t!(label_i18n_slug)}
                </a>
            }
        }
    }
}
