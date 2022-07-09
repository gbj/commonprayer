use crate::utils::time::today;

use super::search_result::{
    PossibleMatch, PossibleMatchOwned, SearchResult, SearchResultContent, SearchResultLink,
};
use calendar::{
    lff2018::LFF_BIOS, Date, Feast, HolyDayId, BCP1979_CALENDAR, LFF2018_CALENDAR, LFF2018_FEASTS,
};
use date_time_parser::DateParser;
use hymnal::{Hymn, Hymnal};
use itertools::Itertools;
use language::Language;
use lazy_static::lazy_static;
use library::{
    lff2018::collects::{LFF_COLLECTS_CONTEMPORARY, LFF_COLLECTS_TRADITIONAL},
    CollectId, CommonPrayer, Contents, Library,
};
use liturgy::{Document, Psalm, Slug, SlugPath};
use psalter::bcp1979::BCP1979_PSALTER;
use reference_parser::{BibleReference, BibleVerse, BibleVersePart, Book};
use regex::Regex;

lazy_static! {
    pub static ref REMOVE_SPACES_AND_PUNCTUATION: Regex = Regex::new(r#"[[[:punct:]][[:space:]]]"#)
        .expect("couldn't compile Regex REMOVE_SPACES_AND_PUNCTUATION");
}

pub fn global_search<L: Library>(
    hymnals: Vec<&Hymnal>,
    q: &str,
    language: Language,
) -> Vec<SearchResult> {
    let raw = q;
    let q = REMOVE_SPACES_AND_PUNCTUATION.replace_all(q, "");
    let q = format!(
        "({})",
        std::iter::Iterator::intersperse_with(
            q.chars().map(|c| c.to_lowercase().to_string()),
            || r#"[[[:punct:]][[:space:]]]*"#.to_string()
        )
        .collect::<String>()
    );
    let q = Regex::new(&q).expect("could not compile search Regex");

    let q_date = DateParser::parse(raw).map(Date::from);
    let q_citation = BibleReference::from(raw);

    // Search for matching hymns
    let hymns = hymnals.into_iter().flat_map(|hymnal| {
        hymnal
            .hymns
            .iter()
            .filter_map(|hymn| hymn.search_in(raw, &q, &q_date, &q_citation, language, None, None))
    });

    // Search for matching holy days
    let holy_days = LFF2018_FEASTS.iter().filter_map(|(_, feast, ..)| {
        feast.search_in(raw, &q, &q_date, &q_citation, language, None, None)
    });

    // Search through document TOC, including both the label for containers and the leaf nodes of documents
    let contents = CommonPrayer::contents();
    let documents = contents
        .flatten()
        .dedup_by(|(slug_path_a, _), (slug_path_b, _)| slug_path_a == slug_path_b)
        .flat_map(|(slug_path, c)| {
            search_contents(c, slug_path, raw, &q, &q_date, &q_citation, language)
        });

    // Search within Psalter
    let psalms = BCP1979_PSALTER.psalms.iter().flat_map(|(_, psalm)| {
        psalm.search_in(raw, &q, &q_date, &q_citation, language, None, None)
    });

    documents
        .chain(psalms)
        .chain(holy_days)
        .chain(hymns)
        .collect()
}

trait Searchable {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        q_date: &Option<Date>,
        q_citation: &BibleReference,
        language: Language,
        slug_path: Option<SlugPath>,
        path: Option<Vec<usize>>,
    ) -> Option<SearchResult>;
}

macro_rules! match_field {
    ($q:ident, $raw:ident, $field:expr, $has_match:ident, $cumulative_score:ident) => {{
        let possible_match = fuzzy_string_match($field, $raw, $q);
        if let PossibleMatch::Matched { score, .. } = &possible_match {
            $has_match = true;
            $cumulative_score += score;
        }
        possible_match
    }};
}

fn fuzzy_string_match<'a>(text: &'a str, raw_q: &str, q: &Regex) -> PossibleMatch<'a> {
    let lower_text = text.to_lowercase();
    let captures = q.captures(&lower_text);

    match captures {
        None => PossibleMatch::None(text),
        Some(captures) => {
            let mut captures = captures.iter();
            let total_match = captures.next().flatten().expect(
                "first item in Captures iter should exist and be the whole matched capture",
            );

            let matched_str = &text[total_match.range()];
            let score = strsim::jaro_winkler(raw_q, matched_str);

            PossibleMatch::Matched {
                original: text,
                score,
                range: total_match.range(),
            }
        }
    }
}

impl Searchable for Hymn {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        _q_date: &Option<Date>,
        _q_citation: &BibleReference,
        _language: Language,
        _slug_path: Option<SlugPath>,
        _path: Option<Vec<usize>>,
    ) -> Option<SearchResult> {
        let mut has_match = false;
        let mut cumulative_score = 0.0;

        let formatted_number = self.number.to_string();
        let number = match_field!(q, raw, &formatted_number, has_match, cumulative_score);
        let title = match_field!(q, raw, &self.title, has_match, cumulative_score);
        let tune = match_field!(q, raw, &self.tune, has_match, cumulative_score);
        let meter = match_field!(q, raw, &self.meter, has_match, cumulative_score);
        let authors = match_field!(q, raw, &self.authors, has_match, cumulative_score);
        let composers = match_field!(q, raw, &self.composers, has_match, cumulative_score);
        let text = match_field!(q, raw, &self.text, has_match, cumulative_score);

        if has_match {
            Some(SearchResult {
                score: cumulative_score,
                link: SearchResultLink::Hymn(self.source, self.number),
                content: SearchResultContent::Hymn {
                    hymnal: self.source,
                    number: (self.number, !matches!(number, PossibleMatch::None(_))),
                    title: title.into(),
                    tune: tune.into(),
                    meter: meter.into(),
                    authors: authors.into(),
                    composers: composers.into(),
                    text: text.into(),
                },
            })
        } else {
            None
        }
    }
}

impl Searchable for Feast {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        q_date: &Option<Date>,
        _q_citation: &BibleReference,
        language: Language,
        _slug_path: Option<SlugPath>,
        _path: Option<Vec<usize>>,
    ) -> Option<SearchResult> {
        let mut has_match = false;
        let mut cumulative_score = 0.0;

        let bcp_name = BCP1979_CALENDAR
            .feast_name(*self, language)
            .unwrap_or_default();

        let lff_name = LFF2018_CALENDAR
            .feast_name(*self, language)
            .unwrap_or_default();

        let name = match match_field!(q, raw, &bcp_name, has_match, cumulative_score) {
            PossibleMatch::Matched {
                original,
                score,
                range,
            } => PossibleMatch::Matched {
                original,
                score,
                range,
            },
            PossibleMatch::None(_) => {
                match_field!(q, raw, &lff_name, has_match, cumulative_score)
            }
        };

        let feast_date = LFF2018_CALENDAR
            .holy_days
            .iter()
            .find(|(_, s_feast, _, _)| s_feast == self)
            .and_then(|(id, _, _, _)| match id {
                HolyDayId::Date(month, day) => Some(Date::from_ymd(today().year(), *month, *day)),
                _ => None,
            });

        let date = match (feast_date, q_date) {
            (Some(f_date), Some(q_date)) if f_date == *q_date => {
                has_match = true;
                cumulative_score += 2.0;
                Some((
                    f_date,
                    format!("{} {}", language.month_name(f_date.month()), f_date.day()),
                    true,
                ))
            }
            (Some(f_date), _) => Some((
                f_date,
                format!("{} {}", language.month_name(f_date.month()), f_date.day()),
                false,
            )),
            _ => None,
        };

        let bio = LFF_BIOS
            .iter()
            .find(|(feast, _)| feast == self)
            .map(|(_, bio)| *bio)
            .unwrap_or_default();
        let bio = match_field!(q, raw, bio, has_match, cumulative_score);

        let collect1 = LFF_COLLECTS_TRADITIONAL
            .iter()
            .find(|(id, _)| *id == CollectId::Feast(*self))
            .map(|(_, data)| data.document.as_text())
            .unwrap_or_default();
        let collect1 = match_field!(q, raw, &collect1, has_match, cumulative_score);

        let collect2 = LFF_COLLECTS_CONTEMPORARY
            .iter()
            .find(|(id, _)| *id == CollectId::Feast(*self))
            .map(|(_, data)| data.document.as_text())
            .unwrap_or_default();
        let collect2 = match_field!(q, raw, &collect2, has_match, cumulative_score);

        if has_match {
            Some(SearchResult {
                score: cumulative_score,
                link: SearchResultLink::Feast(*self),
                content: SearchResultContent::Feast {
                    name: name.into(),
                    date,
                    collect1: collect1.into(),
                    collect2: collect2.into(),
                    bio: bio.into(),
                },
            })
        } else {
            None
        }
    }
}

fn search_contents<'a>(
    contents: Contents<'a>,
    slug_path: SlugPath,
    raw: &'a str,
    q: &'a Regex,
    q_date: &'a Option<Date>,
    q_citation: &'a BibleReference,
    language: Language,
) -> impl Iterator<Item = SearchResult> + 'a {
    if let Contents::Document(document) = contents {
        Box::new(
            document
                .flatten_with_path(true)
                .filter_map(move |(path, document)| {
                    document.search_in(
                        raw,
                        q,
                        q_date,
                        q_citation,
                        language,
                        Some(slug_path.clone()),
                        Some(path),
                    )
                }),
        ) as Box<dyn Iterator<Item = SearchResult>>
    } else if let Contents::MultiDocument { documents, .. } = contents {
        let document_results = documents
            .into_iter()
            .enumerate()
            .flat_map(move |(idx, doc)| {
                let subdocs = doc.flatten();
                doc.search_in(
                    raw,
                    q,
                    q_date,
                    q_citation,
                    language,
                    Some(slug_path.clone()),
                    Some(vec![idx]),
                )
                .into_iter()
                .chain(subdocs.iter().filter_map({
                    let slug_path = slug_path.clone();
                    move |doc| {
                        doc.search_in(
                            raw,
                            q,
                            q_date,
                            q_citation,
                            language,
                            Some(slug_path.clone()),
                            Some(vec![idx]),
                        )
                    }
                }))
                .collect::<Vec<_>>()
            });
        Box::new(document_results) as Box<dyn Iterator<Item = SearchResult>>
    } else {
        Box::new(
            contents
                .search_in(raw, q, q_date, q_citation, language, Some(slug_path), None)
                .into_iter(),
        )
    }
}

// Only searches metadata -- search algorithm will handle children
impl<'a> Searchable for Contents<'a> {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        q_date: &Option<Date>,
        q_citation: &BibleReference,
        language: Language,
        slug_path: Option<SlugPath>,
        path: Option<Vec<usize>>,
    ) -> Option<SearchResult> {
        match self {
            Contents::Category { label, .. } => contents_match_on(label, raw, q, slug_path),
            Contents::Sections { label, .. } => contents_match_on(label, raw, q, slug_path),
            Contents::Document(document) => {
                document.search_in(raw, q, q_date, q_citation, language, slug_path, path)
            }
            Contents::ByVersion { label, .. } => contents_match_on(label, raw, q, slug_path),
            Contents::MultiDocument { label, .. } => contents_match_on(label, raw, q, slug_path),
            Contents::Parallels { label, .. } => contents_match_on(label, raw, q, slug_path),
            Contents::Page { label, .. } => contents_match_on(label, raw, q, slug_path),
        }
    }
}

fn contents_match_on(
    label: &str,
    raw_q: &str,
    q: &Regex,
    slug_path: Option<SlugPath>,
) -> Option<SearchResult> {
    let label = fuzzy_string_match(label, raw_q, q);
    match &label {
        PossibleMatch::Matched { score, .. } => Some(SearchResult {
            score: *score,
            link: SearchResultLink::Document(slug_path.unwrap_or_default(), vec![]),
            content: SearchResultContent::Contents {
                label: label.into(),
            },
        }),
        PossibleMatch::None(_) => None,
    }
}

impl Searchable for Document {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        _q_date: &Option<Date>,
        _q_citation: &BibleReference,
        _language: Language,
        slug_path: Option<SlugPath>,
        path: Option<Vec<usize>>,
    ) -> Option<SearchResult> {
        let mut has_match = false;
        let mut cumulative_score = 0.0;

        let label: PossibleMatchOwned = match self.best_label() {
            Some(label) => match_field!(q, raw, &label, has_match, cumulative_score).into(),
            None => PossibleMatchOwned::None(
                std::iter::Iterator::intersperse_with(
                    slug_path
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|slug| match slug {
                            Slug::Version(version) => version.to_string(),
                            Slug::Canticle(id) => {
                                t!("daily_readings.canticle", number = &id.to_string())
                            }
                            _ => t!(&format!("slug.{:?}", slug)),
                        }),
                    || String::from(" 〉"),
                )
                .collect(),
            ),
        };

        let citation = self.as_citation().unwrap_or_default();
        let citation = match_field!(q, raw, &citation, has_match, cumulative_score);

        let metadata = self.as_metadata_text();
        let metadata = match_field!(q, raw, &metadata, has_match, cumulative_score);

        let text = self.as_text();
        let text = match_field!(q, raw, &text, has_match, cumulative_score);

        if has_match {
            Some(SearchResult {
                score: cumulative_score,
                link: SearchResultLink::Document(
                    slug_path.unwrap_or_default(),
                    path.unwrap_or_default(),
                ),
                content: SearchResultContent::Document {
                    version: self.version,
                    label: label.into(),
                    citation: citation.into(),
                    metadata: metadata.into(),
                    text: text.into(),
                },
            })
        } else {
            None
        }
    }
}

impl Searchable for Psalm {
    fn search_in(
        &self,
        raw: &str,
        q: &Regex,
        q_date: &Option<Date>,
        q_citation: &BibleReference,
        language: Language,
        slug_path: Option<SlugPath>,
        path: Option<Vec<usize>>,
    ) -> Option<SearchResult> {
        let mut has_match = false;
        let mut cumulative_score = 0.0;

        let number_raw = t!("daily_readings.psalm", number = &self.number.to_string());
        let mut number = match_field!(q, raw, &number_raw, has_match, cumulative_score);
        // if doesn't directly match number, try matching query parsed as Biblical citation
        // against the psalm number
        if let PossibleMatch::None(_) = number {
            let filtered = self.filtered_sections();
            if filtered
                .iter()
                .flat_map(|section| section.verses.iter())
                .any(|verse| {
                    q_citation.contains(BibleVerse {
                        book: Book::Psalms,
                        chapter: self.number.into(),
                        verse: verse.number.into(),
                        verse_part: BibleVersePart::All,
                    })
                })
            {
                has_match = true;
                cumulative_score += 1.0;
                number = PossibleMatch::Matched {
                    original: &number_raw,
                    score: 1.0,
                    range: 0..number_raw.len(),
                };
            }
        }

        let metadata = self.as_metadata_text();
        let metadata = match_field!(q, raw, &metadata, has_match, cumulative_score);

        let text = self.as_text();
        let text = match_field!(q, raw, &text, has_match, cumulative_score);

        if has_match {
            Some(SearchResult {
                score: cumulative_score,
                link: SearchResultLink::Psalm(self.number),
                content: SearchResultContent::Psalm {
                    number: number.into(),
                    metadata: metadata.into(),
                    text: text.into(),
                },
            })
        } else {
            None
        }
    }
}
