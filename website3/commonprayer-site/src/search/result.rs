use std::ops::Range;

use calendar::{Date, Feast};
use hymnal::{HymnNumber, Hymnals};
use leptos::*;
use liturgy::{SlugPath, Version};
use serde::{Deserialize, Serialize};

use crate::{i18n::use_i18n, icon::Icon};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SearchResult {
    pub score: i32,
    pub link: SearchResultLink,
    pub content: SearchResultContent,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SearchResultLink {
    Document(SlugPath, Vec<usize>),
    Psalm(u8),
    Feast(Feast),
    Hymn(Hymnals, HymnNumber),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PossibleMatch<'a> {
    Matched {
        original: &'a str,
        score: i32,
        range: Range<usize>,
    },
    None(&'a str),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PossibleMatchOwned {
    Matched {
        original: String,
        score: i32,
        range: Range<usize>,
    },
    None(String),
}

impl<'a> From<PossibleMatch<'a>> for PossibleMatchOwned {
    fn from(pm: PossibleMatch) -> Self {
        match pm {
            PossibleMatch::Matched {
                original,
                score,
                range,
            } => PossibleMatchOwned::Matched {
                original: original.to_string(),
                score,
                range,
            },
            PossibleMatch::None(s) => PossibleMatchOwned::None(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SearchResultContent {
    Hymn {
        hymnal: Hymnals,
        number: (HymnNumber, bool),
        title: PossibleMatchOwned,
        tune: PossibleMatchOwned,
        meter: PossibleMatchOwned,
        authors: PossibleMatchOwned,
        composers: PossibleMatchOwned,
        text: PossibleMatchOwned,
    },
    Feast {
        name: PossibleMatchOwned,
        date: Option<(Date, String, bool)>,
        collect1: PossibleMatchOwned,
        collect2: PossibleMatchOwned,
        bio: PossibleMatchOwned,
    },
    Document {
        version: Version,
        label: PossibleMatchOwned,
        citation: PossibleMatchOwned,
        metadata: PossibleMatchOwned,
        text: PossibleMatchOwned,
    },
    Psalm {
        number: PossibleMatchOwned,
        metadata: PossibleMatchOwned,
        text: PossibleMatchOwned,
    },
    Contents {
        label: PossibleMatchOwned,
    },
}

#[component]
pub fn ShowSearchResult(cx: Scope, content: SearchResult) -> Element {
    let (t, _, _) = use_i18n(cx);

    let class = match content.content {
        SearchResultContent::Hymn { .. } => "content hymn-listing",
        _ => "content",
    };

    let href = match &content.link {
        SearchResultLink::Document(slug, path) => {
            let path = if path.is_empty() {
                "".to_string()
            } else {
                format!(
                    "#{}",
                    path.iter()
                        .map(|n| n.to_string())
                        .intersperse_with(|| String::from("-"))
                        .collect::<String>()
                )
            };
            format!("../document/{}{}", slug, path)
        }
        SearchResultLink::Feast(feast) => {
            format!("../readings/holy-day/?id={:?}", feast)
        }
        SearchResultLink::Hymn(hymnal, number) => {
            format!("../hymn/{:?}/{}", hymnal, number)
        }
        SearchResultLink::Psalm(number) => {
            format!("../psalm/{}", number)
        }
    };

    let icon_src = match content.content {
        SearchResultContent::Hymn { .. } => Icon::Music,
        SearchResultContent::Feast { .. } => Icon::Harp,
        SearchResultContent::Document { .. } => Icon::Book,
        SearchResultContent::Psalm { .. } => Icon::Harp,
        SearchResultContent::Contents { .. } => Icon::Folder,
    }
    .to_string();

    let icon_label = match content.content {
        SearchResultContent::Hymn { .. } => t("search-hymn"),
        SearchResultContent::Feast { .. } => t("search-holy_day"),
        SearchResultContent::Document { .. } => t("search-document"),
        SearchResultContent::Psalm { .. } => t("search-psalm"),
        SearchResultContent::Contents { .. } => t("search-contents"),
    };

    let title = match &content.content {
        SearchResultContent::Hymn { title, .. } => title,
        SearchResultContent::Feast { name, .. } => name,
        SearchResultContent::Document { label, .. } => label,
        SearchResultContent::Psalm { number, .. } => number,
        SearchResultContent::Contents { label } => label,
    }
    .clone();

    view! {
        <li class="search-result">
            <div class=class>
                <div class="primary">
                    <img class="icon" src=icon_src alt=icon_label/>
                    <a class="title" href=href>
                        <ShowPossibleMatch value=title />
                    </a>
                </div>
                <div class="secondary"></div>
            </div>
        </li>
    }
}

#[component]
fn ShowPossibleMatch(cx: Scope, value: PossibleMatchOwned) -> Element {
    const MAX_UNTRUNCATED: usize = 80;

    let before_match = match &value {
        PossibleMatchOwned::None(value) => value.clone(),
        PossibleMatchOwned::Matched {
            original, range, ..
        } => {
            if range.start > MAX_UNTRUNCATED {
                let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                truncated.push('…');
                for c in original
                    .chars()
                    .skip(range.start - (MAX_UNTRUNCATED - 1))
                    .take(MAX_UNTRUNCATED - 1)
                {
                    truncated.push(c);
                }
                truncated
            } else {
                original.chars().take(range.start).collect::<String>()
            }
        }
    };

    let matched_str = match &value {
        PossibleMatchOwned::None(_) => Default::default(),
        PossibleMatchOwned::Matched {
            original, range, ..
        } => original
            .chars()
            .skip(range.start)
            .take(range.end - range.start)
            .collect::<String>(),
    };

    let after_match = match &value {
        PossibleMatchOwned::None(_) => Default::default(),
        PossibleMatchOwned::Matched {
            original, range, ..
        } => {
            let remaining_after_match_ends = original.len() - range.end;
            if remaining_after_match_ends > MAX_UNTRUNCATED {
                let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                for c in original.chars().skip(range.end).take(MAX_UNTRUNCATED - 1) {
                    truncated.push(c);
                }
                truncated.push('…');
                truncated
            } else {
                original.chars().skip(range.end).collect::<String>()
            }
        }
    };

    view! {
            <div>
                {before_match}
                <span class="match">{matched_str}</span>
                {after_match}
            </div>
    }
}

/*
impl SearchResult {
    fn view(&self, locale: &str) -> Element {
        let class = match &self.content {
            SearchResultContent::Hymn { .. } => "content hymn-listing",
            _ => "content",
        };



        let content = match &self.content {
            SearchResultContent::Hymn {
                hymnal,
                number,
                title,
                tune,
                authors,
                composers,
                meter,
                text,
            } => view! {
                <>
                    <div class="primary">
                        <img class="icon" src={Icon::Music.to_string()} alt={t!("search_page.hymn")}/>
                        <a class="number" class:match={number.1} href={&href}>
                            {hymnal.to_string()} " " {number.0.to_string()}
                        </a>
                        <a class="title" href={&href}>{title}</a>
                        <span class="tune">{tune}</span>
                    </div>
                    <div class="secondary">
                        <div>
                            <span class="list-field author">{authors}</span>
                            <span class="list-field composer">{composers}</span>
                        </div>
                        <span class="list-field meter">{meter}</span>
                    </div>
                    <div class="text">
                        {if matches!(text, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(text.to_node())
                        }}
                    </div>
                </>
            },
            SearchResultContent::Feast {
                name,
                date,
                collect1,
                collect2,
                bio,
            } => {
                view! {
                    <>
                        <div class="primary">
                            <img class="icon" src={Icon::Halo.to_string()} alt={t!("search_page.holy_day")}/>
                            <a class="title" href={&href}>{name}</a>
                            {date.as_ref()
                                .map(|date| view! {
                                    <span class="date">{
                                        if date.2 {
                                            view! {
                                                <span class="match">{&date.1}</span>
                                            }
                                        } else {
                                            text(&date.1)
                                        }
                                    }</span>
                                })
                            }
                        </div>
                        <div class="collect">
                            {if matches!(collect1, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(collect1.to_node())
                            }}
                        </div>
                        <div class="collect">
                            {if matches!(collect2, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(collect2.to_node())
                            }}
                        </div>
                        <div class="bio">
                            {if matches!(bio, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(bio.to_node())
                            }}
                        </div>
                    </>
                }
            }
            SearchResultContent::Document {
                version,
                label,
                citation,
                metadata,
                text,
            } => {
                view! {
                    <>
                        <div class="primary">
                            <img class="icon" src={Icon::Book.to_string()} alt={t!("search_page.document")}/>
                            <a class="title" href={&href}>
                                {label}
                            </a>
                            {if *version != Version::BCP1979 {
                                Some(view! {
                                    <span class="version">" (" {version.to_string()} ")"</span>
                                })
                            } else {
                                None
                            }}
                            <span class="citation">{citation}</span>
                        </div>
                        <div class="metadata">
                            {if matches!(metadata, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(metadata.to_node())
                            }}
                        </div>
                        <div class="text">
                            {if matches!(text, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(text.to_node())
                            }}
                        </div>
                    </>
                }
            }
            SearchResultContent::Psalm {
                number,
                metadata,
                text,
            } => {
                view! {
                    <>
                        <div class="primary">
                            <img class="icon" src={Icon::Harp.to_string()} alt={t!("search_page.psalm")}/>
                            <a class="title" href={&href}>{number}</a>
                        </div>
                        <div class="metadata">
                            {if matches!(metadata, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(metadata.to_node())
                            }}
                        </div>
                        <div class="text">
                            {if matches!(text, PossibleMatchOwned::None(_)) {
                                None
                            } else {
                                Some(text.to_node())
                            }}
                        </div>
                    </>
                }
            }
            SearchResultContent::Contents { label } => {
                view! {
                    <>
                        <div class="primary">
                            <img class="icon" src={Icon::Folder.to_string()} alt={t!("search_page.category")}/>
                            <a class="title" href={&href}>{label}</a>
                        </div>
                    </>
                }
            }
        };

        view! {
            <li class="search-result">
                <div class={class}>
                    {content}
                </div>
            </li>
        }
    }
}

const MAX_UNTRUNCATED: usize = 80;

impl PossibleMatchOwned {
    fn to_node(&self) -> Element {
        match self {
            PossibleMatchOwned::None(unmatched_text) => view! { <span>{unmatched_text}</span> },
            PossibleMatchOwned::Matched {
                original, range, ..
            } => {
                let before_match = if range.start > MAX_UNTRUNCATED {
                    let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                    truncated.push('…');
                    for c in original
                        .chars()
                        .skip(range.start - (MAX_UNTRUNCATED - 1))
                        .take(MAX_UNTRUNCATED - 1)
                    {
                        truncated.push(c);
                    }
                    truncated
                } else {
                    original.chars().take(range.start).collect::<String>()
                };
                let before_match = if range.start > MAX_UNTRUNCATED {
                    let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                    truncated.push('…');
                    for c in original
                        .chars()
                        .skip(range.start - (MAX_UNTRUNCATED - 1))
                        .take(MAX_UNTRUNCATED - 1)
                    {
                        truncated.push(c);
                    }
                    truncated
                } else {
                    original.chars().take(range.start).collect::<String>()
                };
                let before_match = if range.start > MAX_UNTRUNCATED {
                    let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                    truncated.push('…');
                    for c in original
                        .chars()
                        .skip(range.start - (MAX_UNTRUNCATED - 1))
                        .take(MAX_UNTRUNCATED - 1)
                    {
                        truncated.push(c);
                    }
                    truncated
                } else {
                    original.chars().take(range.start).collect::<String>()
                };

                let matched_str = original
                    .chars()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect::<String>();

                let remaining_after_match_ends = original.len() - range.end;
                let after_match = if remaining_after_match_ends > MAX_UNTRUNCATED {
                    let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                    for c in original.chars().skip(range.end).take(MAX_UNTRUNCATED - 1) {
                        truncated.push(c);
                    }
                    truncated.push('…');
                    truncated
                } else {
                    original.chars().skip(range.end).collect::<String>()
                };

                view! {
                        <div>
                            {before_match}
                            <span class="match">{matched_str}</span>
                            {after_match}
                        </div>
                }
            }
        }
    }
}
 */
