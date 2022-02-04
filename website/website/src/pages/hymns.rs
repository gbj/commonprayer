use crate::components::{header, SearchBar, SegmentButton};
use episcopal_api::hymnal::*;
use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
    number: Option<HymnNumber>,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum HymnalPageProps {
    Hymnal(Vec<Hymnal>),
    Hymn(HymnalMetadata, Hymn),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HymnalMetadata {
    pub id: Hymnals,
    pub title: String,
    pub subtitle: String,
    pub copyright: String,
}

pub fn hymnal() -> Page<HymnalPageProps, HymnalPageParams> {
    Page::new("hymnal")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(get_static_props)
        .build_paths_fn(get_static_paths)
}

pub fn head(_locale: &str, props: &HymnalPageProps) -> View {
    let title = match props {
        HymnalPageProps::Hymnal(hymnals) => {
            if hymnals.len() == 1 {
                hymnals[0].title.clone()
            } else {
                t!("menu.hymnal")
            }
        }
        HymnalPageProps::Hymn(_, hymn) => format!("{} {}", hymn.number, hymn.title),
    };

    view! {
        <>
            <title>{title} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/hymnal.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "{hymnal}".into(), "{hymnal}/{number}".into()]
}

pub fn get_static_props(
    _locale: &str,
    _path: &str,
    params: HymnalPageParams,
) -> Option<HymnalPageProps> {
    Some(match (params.hymnal, params.number) {
        (None, None) => {
            HymnalPageProps::Hymnal(vec![HYMNAL_1982.clone(), LEVAS.clone(), WLP.clone()])
        }
        (Some(hymnal), None) => HymnalPageProps::Hymnal(vec![hymnal.into()]),
        (hymnal, Some(number)) => {
            let hymnal: Hymnal = hymnal.unwrap_or_default().into();
            let metadata = HymnalMetadata {
                id: hymnal.id,
                title: hymnal.title,
                subtitle: hymnal.subtitle,
                copyright: hymnal.copyright,
            };
            let hymn = hymnal.hymns.iter().find(|s_hymn| s_hymn.number == number)?;
            HymnalPageProps::Hymn(metadata, hymn.clone())
        }
    })
}

pub fn body(locale: &str, props: &HymnalPageProps) -> View {
    match props {
        HymnalPageProps::Hymnal(hymnals) => hymnal_body(locale, hymnals),
        HymnalPageProps::Hymn(hymnal, hymn) => hymn_body(locale, hymnal, hymn),
    }
}

pub fn hymnal_body(locale: &str, hymnals: &[Hymnal]) -> View {
    let search_bar = SearchBar::new();
    let hymnal_choice = SegmentButton::new_with_default_value(
        "search-hymnal",
        Some(t!("menu.hymnal")),
        [
            (None, t!("hymnal.any"), None),
            (Some(Hymnals::Hymnal1982), t!("hymnal.h82_abbrev"), None),
            (Some(Hymnals::LEVAS), t!("hymnal.levas_abbrev"), None),
            (Some(Hymnals::WLP), t!("hymnal.wlp_abbrev"), None),
        ],
        if hymnals.len() == 1 {
            hymnals.get(0).map(|h| h.id)
        } else {
            None
        },
    );

    let hymnal_tables = View::Fragment(
        hymnals
            .iter()
            .map({
                let search = search_bar.value.clone();
                let hymnal_choice = hymnal_choice.value.clone();
                move |hymnal| {
                    let title = view! { <h2>{&hymnal.title}</h2> };
                    let subtitle = if !hymnal.subtitle.is_empty() {
                        view! { <h3>{&hymnal.subtitle}</h3> }
                    } else {
                        View::Empty
                    };
                    let copyright = view! { <p class="copyright">{&hymnal.copyright}</p> };

                    let hymns = View::Fragment(
                        hymnal
                            .hymns
                            .iter()
                            .map(|hymn| {
                                let number = match hymn.number {
                                    HymnNumber::S(n) => format!("S{}", n),
                                    HymnNumber::H(n) => n.to_string(),
                                };

                                let tune_name = if hymn.tune.starts_with('[') {
                                    ""
                                } else {
                                    &hymn.tune
                                }
                                .to_lowercase();

                                let hidden = search
                                    .stream()
                                    .map({
                                        let number = number.clone();
                                        let title = hymn.title.clone();
                                        let tune = hymn.tune.clone();
                                        move |search| {
                                            let search = search.to_lowercase();
                                            !search.is_empty()
                                                && !number.to_lowercase().contains(&search)
                                                && !title.to_lowercase().contains(&search)
                                                && !tune.to_lowercase().contains(&search)
                                        }
                                    })
                                    .boxed_local();

                                let link =
                                    format!("/{}/hymnal/{:#?}/{}", locale, hymnal.id, hymn.number);

                                view! {
                                    <dyn:tr class:hidden={hidden}>
                                        <td><a href={&link}>{number}</a></td>
                                        <td><a href={&link}>{&hymn.title}</a></td>
                                        <td class="tune">{&tune_name}</td>
                                    </dyn:tr>
                                }
                            })
                            .collect(),
                    );

                    let id = hymnal.id;
                    let hidden = hymnal_choice
                        .stream()
                        .map(move |choice| choice.is_some() && choice != Some(id))
                        .boxed_local();

                    view! {
                        <dyn:section class:hidden={hidden}>
                            {title}
                            {subtitle}
                            {copyright}
                            <table>
                                {hymns}
                            </table>
                        </dyn:section>
                    }
                }
            })
            .collect(),
    );

    view! {
        <>
            {header(locale, &t!("menu.hymnal"))}
            <main>
                {search_bar.view()}
                {if hymnals.len() == 1 {
                    View::Empty
                } else {
                    hymnal_choice.view()
                }}
                {hymnal_tables}
            </main>
        </>
    }
}

fn hymn_body(locale: &str, hymnal: &HymnalMetadata, hymn: &Hymn) -> View {
    let hymnary_link = format!(
        "https://hymnary.org/hymn/{}/{}",
        match hymnal.id {
            Hymnals::Hymnal1982 => "EH1982",
            Hymnals::LEVAS => "LEVAS1993",
            Hymnals::WLP => "WLP1997",
        },
        hymn.number
    );

    view! {
        <>
            {header(locale, &format!("{} {}", hymn.number, hymn.title))}
            <main>
                <h2>
                    <a href={&format!("/{}/hymnal/{:#?}", locale, hymnal.id)}>
                        {&hymnal.title}
                    </a>
                    " "
                    {hymn.number.to_string()}
                </h2>
                <h3>{&hymn.title}</h3>
                <h4 class="tune">{hymn.tune.to_lowercase()}</h4>
                <a class="hymnary-link" href={hymnary_link} target="_blank">"Hymnary.org"</a>
            </main>
        </>
    }
}
