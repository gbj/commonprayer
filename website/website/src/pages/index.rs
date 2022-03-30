use crate::components::header;
use leptos::*;
use rust_i18n::t;

pub fn index() -> Page<(), (), ()> {
    Page::new("index")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(|_, _, _| Some(()))
        .static_page()
        .incremental_generation()
}

fn head(_locale: &str, _props: &(), _render_state: &()) -> View {
    view! {
        <>
            <title>{t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
        </>
    }
}

fn body(locale: &str, _props: &(), _render_state: &()) -> View {
    view! {
        <>
            {header(locale, &t!("common_prayer"))}
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
                        {make_link(locale, "daily-office", "toc.daily_office", false)}
                        <ul>
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
                                {make_link(locale, "document/office/noonday-prayer/RiteII", "toc.noonday_prayer", false)}
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
                                {make_link(locale, "document/office/compline/RiteII", "toc.compline", false)}
                            </li>
                            <li>
                                {make_link(locale, "canticle-table", "toc.canticle_table", false)}
                            </li>
                        </ul>
                    </li>
                    <li>
                        {make_link(locale, "document/litany", "toc.great_litany", true)}
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
                        {make_link(locale, "document/baptism", "toc.holy_baptism", true)}
                    </li>
                    <li>
                        {make_link(locale, "document/eucharist", "toc.holy_eucharist", true)}
                    </li>
                    <li>
                        {t!("toc.pastoral_offices")}
                        <ul>
                            <li>
                                {make_link(locale, "document/marriage", "toc.marriage", false)}
                            </li>
                            <li>
                                {make_link(locale, "document/burial", "toc.burial", false)}
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
) -> View {
    if work_in_progress {
        view! {
            <>
                {t!(label_i18n_slug)}
                " "
                <em>{t!("toc.work_in_progress")}</em>
            </>
        }
    } else {
        view! {
            <a href={format!("/{}/{}", locale, path)}>
                {t!(label_i18n_slug)}
            </a>
        }
    }
}
