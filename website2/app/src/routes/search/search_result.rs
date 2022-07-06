use std::ops::Range;

use crate::{Icon, WebView};
use calendar::{Date, Feast};
use hymnal::{HymnNumber, Hymnals};
use itertools::Itertools;
use leptos2::*;
use liturgy::{SlugPath, Version};

pub struct SearchResult {
    pub score: f64,
    pub link: SearchResultLink,
    pub content: SearchResultContent,
}

pub enum SearchResultLink {
    Document(SlugPath, Vec<usize>),
    Feast(Feast),
    Hymn(Hymnals, HymnNumber),
}

pub enum PossibleMatch<'a> {
    Matched {
        original: &'a str,
        score: f64,
        range: Range<usize>,
    },
    None(&'a str),
}

pub enum PossibleMatchOwned {
    Matched {
        original: String,
        score: f64,
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
    Contents {
        label: PossibleMatchOwned,
    },
}

impl WebView for SearchResult {
    fn view(&self, locale: &str) -> leptos2::Node {
        let class = match &self.content {
            SearchResultContent::Hymn { .. } => "content hymn-listing",
            _ => "content",
        };

        let href = match &self.link {
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
                format!("/{}/document/{}{}", locale, slug, path)
            }
            SearchResultLink::Feast(feast) => {
                format!("/{}/readings/holy-day/?id={:?}", locale, feast)
            }
            SearchResultLink::Hymn(hymnal, number) => {
                format!("/{}/hymn/{:?}/{}", locale, hymnal, number)
            }
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
                            {if matches!(text, PossibleMatchOwned::None(_)) {
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

impl<'a> StaticView for &PossibleMatchOwned {
    fn to_node(&self) -> Node {
        match self {
            PossibleMatchOwned::None(unmatched_text) => text(unmatched_text),
            PossibleMatchOwned::Matched {
                original, range, ..
            } => {
                let before_match = if range.start > MAX_UNTRUNCATED {
                    let mut truncated = String::with_capacity(MAX_UNTRUNCATED);
                    truncated.push_str("...");
                    for c in original
                        .chars()
                        .skip(range.start - (MAX_UNTRUNCATED - 3))
                        .take(MAX_UNTRUNCATED - 3)
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
                    for c in original.chars().skip(range.end).take(MAX_UNTRUNCATED - 3) {
                        truncated.push(c);
                    }
                    truncated.push_str("...");
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
