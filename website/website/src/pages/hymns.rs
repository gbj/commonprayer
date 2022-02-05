use crate::components::{header, SearchBar, SegmentButton, Icon};
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
                                        let number = strip_non_word_characters(&hymn.number.to_string().to_lowercase());
                                        let title = strip_non_word_characters(&hymn.title.to_lowercase());
                                        let tune = strip_non_word_characters(&hymn.tune.to_lowercase());
                                        let authors = strip_non_word_characters(&hymn.authors.to_lowercase());
                                        let composers = strip_non_word_characters(&hymn.composers.to_lowercase());
                                        let meter = hymn.meter.clone();
                                        move |orig_search| {
                                            let search = strip_non_word_characters(&orig_search.to_lowercase());
                                            !(search.is_empty() || number.contains(&search) || title.contains(&search) || tune.contains(&search) || authors.contains(&search) || composers.contains(&search) || meter.contains(&orig_search))
                                        }
                                    })
                                    .boxed_local();

                                let link =
                                    format!("/{}/hymnal/{:#?}/{}", locale, hymnal.id, hymn.number);

                                view! {
                                    <dyn:article
                                        class="hymn-listing"
                                        class:hidden={hidden}
                                    >
                                        <div class="primary">
                                            <span class="music-available">
                                                {if hymn.copyright_restriction { 
                                                    View::Empty 
                                                } else {
                                                    view! { <img src={Icon::Music.to_string()} alt={t!("hymnal.music_available")}/> }
                                                }}
                                            </span>
                                            <a class="number" href={&link}>{number}</a>
                                            <a class="title" href={&link}>{&hymn.title}</a>
                                            <span class="tune">{&tune_name}</span>
                                        </div>
                                        <div class="secondary">
                                            <div>
                                                {if hymn.authors.is_empty() {
                                                    View::Empty
                                                } else {
                                                    view! {
                                                        <span class="list-field author">
                                                            <span class="label">{t!("hymnal.text")} ": "</span>
                                                            {&hymn.authors}
                                                        </span>
                                                }}}
                                                {if hymn.composers.is_empty() {
                                                    View::Empty
                                                } else {
                                                    view! {
                                                        <span class="list-field composer">
                                                            <span class="label">{t!("hymnal.music")} ": "</span>
                                                            {&hymn.composers}
                                                        </span>
                                                }}}
                                            </div>
                                            <span class="list-field meter">{&hymn.meter}</span>
                                        </div>
                                    </dyn:article>
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
    let hymnary_hymnal_id = match hymnal.id {
        Hymnals::Hymnal1982 => "EH1982",
        Hymnals::LEVAS => "LEVS1993",
        Hymnals::WLP => "WLP1997",
    };
    let hymnary_hymn_link = format!(
        "https://hymnary.org/hymn/{}/{}",
        &hymnary_hymnal_id, hymn.number
    );

    let image_expanded = Behavior::new(false);

    // page scrolling through hymnal
    let initial_page : i32 = hymn.page_number.into();
    let page_scan_offset = Behavior::new(0);
    let page_scan_url = page_scan_offset.stream().map(move |offset| { let current_page = initial_page + offset;
        format!(
        "https://hymnary.org/page/fetch/{}/{}/high",
        &hymnary_hymnal_id, current_page
    )
    }).boxed_local();

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

                // Hymn metadata
                <h3>{&hymn.title}</h3>

                <dl>
                    <dt>{t!("hymnal.tune")}</dt>
                    <dd class="tune">{hymn.tune.to_lowercase()}</dd>
                    {possible_field(&t!("hymnal.first_line"), &hymn.first_line)}
                    {possible_field(&t!("hymnal.text_title"), &hymn.text_title)}
                    {possible_field(&t!("hymnal.refrain_first_line"), &hymn.refrain_first_line)}
                    {possible_field(&t!("hymnal.authors"), &hymn.authors)}
                    {possible_field(&t!("hymnal.composers"), &hymn.composers)}
                    {possible_field(&t!("hymnal.meter"), &hymn.meter)}
                    {possible_field(&t!("hymnal.text_sources"), &hymn.text_sources)}
                    {possible_field(&t!("hymnal.tune_sources"), &hymn.tune_sources)}
                </dl>
                // Link to Hymnary.org
                <p class="hymnary-link">
                    {t!("hymnal.more_info")}
                    " "
                    <a class="hymnary-link" href={hymnary_hymn_link} target="_blank">"Hymnary.org"</a>
                    "."
                </p>

                // Hymn image
                <dyn:div class="overlay"
                    class:expanded={image_expanded.stream().boxed_local()}
                    on:click={
                        let image_expanded = image_expanded.clone();
                        move |_ev: Event| image_expanded.set(!image_expanded.get())
                    }
                ></dyn:div>

                <dyn:div class="page-scan-controls"
                    class:expanded={image_expanded.stream().boxed_local()}
                >
                    <dyn:button
                        class="page-left"
                        on:click={
                            let page_scan_offset = page_scan_offset.clone();
                            move |_ev: Event| {
                                let current_offset = page_scan_offset.get();
                                let current_page = initial_page + current_offset;
                                if current_page > 1 {
                                    page_scan_offset.set(current_offset - 1);
                                }
                            }
                        }
                    >
                        <img src={Icon::Left.to_string()} alt={t!("hymnal.page_back")}/>
                    </dyn:button>
                    <dyn:p class="page-scan-number">
                        {page_scan_offset.stream().map(move |offset| t!("hymnal.page_n", number = &(initial_page + offset).to_string() )).boxed_local()}
                    </dyn:p>
                    <dyn:button
                        class="page-left"
                        on:click=move |_ev: Event| page_scan_offset.set(page_scan_offset.get() + 1)
                    >
                        <img src={Icon::Right.to_string()} alt={t!("hymnal.page_forward")}/>
                    </dyn:button>
                </dyn:div>

                {if hymn.copyright_restriction {
                    view! {
                        <p class="page-scan">{t!("hymnal.copyright_restriction")}</p>
                    }
                } else {
                    view! {
                        <dyn:img
                            src={page_scan_url}
                            alt={t!("hymnal.alt_text")}
                            class="page-scan"
                            class:expanded={image_expanded.stream().boxed_local()}
                            on:click={
                                let image_expanded = image_expanded.clone();
                                move |_ev: Event| image_expanded.set(!image_expanded.get())
                            }
                        />
                    }
                }}

                // Copyright notice in footer
                <footer>
                    {t!("hymnal.copyright_footer")}
                </footer>
            </main>
        </>
    }
}

fn possible_field(label: &str, value: &str) -> View {
    if value.is_empty() {
        View::Empty
    } else {
        view! {
            <>
                <dt>{label}</dt>
                <dd>{escape_italics(value)}</dd>
            </>
        }
    }
}

fn escape_italics(original: &str) -> View {
    View::Fragment(
        original.split("<i>")
            .flat_map(|piece| piece.split("</i>"))
            .enumerate()
            .map(|(idx, piece)| if idx % 2 == 0 {
                View::StaticText(piece.to_string())
            } else {
                // every odd character piece will be *after* a <i> but before a </i>
                view! { <i>{piece}</i> }
            })
            .collect()
    )
}

fn strip_non_word_characters(original: &str) -> String {
    original.chars().filter(|ch| 
        // so that date ranges don't get read as numbers, i.e., "111" should not match "1711-1759"
        ch == &'-'
        // letters
        || ('a'..'z').contains(ch)
        // digits so can search by hymn number
        || ('0'..'9').contains(ch)).collect()
}

