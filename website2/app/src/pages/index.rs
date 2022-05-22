use crate::views::Header;
use leptos2::*;

pub fn index() -> Page<(), ()> {
    Page::new("index")
        .head_fn(head)
        .body_fn(body)
        .state(|_, _, _| Some(()))
        .static_page()
        .incremental_generation()
}

fn head(_locale: &str, _props: &()) -> Vec<Node> {
    view! {
        <>
            <title>{t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
        </>
    }
}

fn body(locale: &str, _props: &()) -> Vec<Node> {
    view! {
        <>
            {Header::new(locale, &t!("common_prayer")).to_node()}
            <main>
                <h1>{t!("common_prayer")}</h1>
                <ul class="toc-menu">
                    <li>
                        {t!("toc.calendar_full")}
                        <ul>
                            <li>
                                {make_link(locale, "calendar/bcp1979", "bcp_1979", false)}
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
            </main>
        </>
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
