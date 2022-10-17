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

    let content = match content.content {
        SearchResultContent::Hymn {
            hymnal,
            number,
            title,
            tune,
            authors,
            composers,
            meter,
            text,
        } => view! { cx, 
            <>
                <div class="primary">
                    <img class="icon" src={Icon::Music.to_string()} alt={t("search-hymn")}/>
                    <a class="number" class:match={number.1} href={&href}>
                        {hymnal.to_string()} " " {number.0.to_string()}
                    </a>
                    <a class="title" href={&href}><ShowPossibleMatch value=title/></a>
                    <span class="tune"><ShowPossibleMatch value=tune/></span>
                </div>
                <div class="secondary">
                    <div>
                        <span class="list-field author"><ShowPossibleMatch value=authors/></span>
                        <span class="list-field composer"><ShowPossibleMatch value=composers/></span>
                    </div>
                    <span class="list-field meter"><ShowPossibleMatch value=meter/></span>
                </div>
                <div class="text">
                    {if matches!(text, PossibleMatchOwned::None(_)) {
                        None
                    } else {
                        Some(view! { cx,  <ShowPossibleMatch value=text/> })
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
            view! { cx, 
                <>
                    <div class="primary">
                        <img class="icon" src={Icon::Halo.to_string()} alt={t("search-holy_day")}/>
                        <a class="title" href={&href}><ShowPossibleMatch value=name/></a>
                        {date.as_ref()
                            .map(|date| view! { cx, 
                                <span class="date">{
                                    if date.2 {
                                        view! { cx, 
                                            <span class="match">{&date.1}</span>
                                        }
                                    } else {
                                        view! { cx, 
                                            <span>{&date.1}</span>
                                        }
                                    }
                                }</span>
                            })
                        }
                    </div>
                    <div class="collect">
                        {if matches!(collect1, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=collect1/> })
                        }}
                    </div>
                    <div class="collect">
                        {if matches!(collect2, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=collect2/> })
                        }}
                    </div>
                    <div class="bio">
                        {if matches!(bio, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=bio/> })
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
            view! { cx, 
                <>
                    <div class="primary">
                        <img class="icon" src={Icon::Book.to_string()} alt={t("search-document")}/>
                        <a class="title" href={&href}>
                             <ShowPossibleMatch value=label/>
                        </a>
                        {if version != Version::BCP1979 {
                            Some(view! { cx, 
                                <span class="version">" (" {version.to_string()} ")"</span>
                            })
                        } else {
                            None
                        }}
                        <span class="citation"><ShowPossibleMatch value=citation/></span>
                    </div>
                    <div class="metadata">
                        {if matches!(metadata, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=metadata/> })
                        }}
                    </div>
                    <div class="text">
                        {if matches!(text, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=text/> })
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
            view! { cx, 
                <>
                    <div class="primary">
                        <img class="icon" src={Icon::Harp.to_string()} alt={t("search-psalm")}/>
                        <a class="title" href={&href}><ShowPossibleMatch value=number/></a>
                    </div>
                    <div class="metadata">
                        {if matches!(metadata, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=metadata/> })
                        }}
                    </div>
                    <div class="text">
                        {if matches!(text, PossibleMatchOwned::None(_)) {
                            None
                        } else {
                            Some(view! { cx,  <ShowPossibleMatch value=text/> })
                        }}
                    </div>
                </>
            }
        }
        SearchResultContent::Contents { label } => {
            view! { cx, 
                <>
                    <div class="primary">
                        <img class="icon" src={Icon::Folder.to_string()} alt={t("search-category")}/>
                        <a class="title" href={&href}><ShowPossibleMatch value=label/></a>
                    </div>
                </>
            }
        }
    };

    view! { cx, 
        <li class="search-result">
            <div class=class>
                {content}
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

    view! { cx, 
            <div>
                {before_match}
                <span class="match">{matched_str}</span>
                {after_match}
            </div>
    }
}
