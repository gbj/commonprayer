#![feature(iter_intersperse)]

use calendar::LiturgicalDayId;
use language::Language;
use liturgy::*;
use status::Status;

pub trait ToRustCode {
    fn to_rust_code(&self, start_tabs: usize) -> String;
}

impl ToRustCode for Document {
    fn to_rust_code(&self, start_tabs: usize) -> String {
        let condition = self
            .condition
            .as_ref()
            .map(|cond| format!(".condition({})", cond.to_rust_code(start_tabs)))
            .unwrap_or_default();

        // Metadata
        let label = self
            .label
            .as_ref()
            .map(|value| format!("\t.label({:?})", value))
            .unwrap_or_default();
        let version_label = self
            .version_label
            .as_ref()
            .map(|value| format!("\t.version_label({:?})", value))
            .unwrap_or_default();
        let explainer = self
            .explainer
            .as_ref()
            .map(|value| format!("\t.explainer({:?})", value))
            .unwrap_or_default();
        let subtitle = self
            .subtitle
            .as_ref()
            .map(|value| format!("\t.subtitle({:?})", value))
            .unwrap_or_default();
        let language = if Language::is_default(&self.language) {
            String::default()
        } else {
            format!("\t.language(Language::{})", self.language)
        };
        let source = self
            .source
            .map(|reference| {
                let source_str: &'static str = reference.source.into();
                if reference.source == Source::BCP1979 {
                    format!("\t.page({})", reference.page)
                } else {
                    format!(
                        "\t.source(Reference {{
                source: Source::{},
                page: {}
            }})",
                        source_str, reference.page
                    )
                }
            })
            .unwrap_or_default();
        let alternate_sources = if self.alternate_sources.is_empty() {
            String::new()
        } else {
            self.alternate_sources
                .iter()
                .map(|reference| {
                    format!(
                        "\t.alternate_source(Reference {{
                source: Source::{},
                page: {}
            }})",
                        reference.source, reference.page
                    )
                })
                .intersperse_with(|| String::from("\n"))
                .collect()
        };
        let status = if Status::is_default(&self.status) {
            String::default()
        } else {
            format!("\t.status(Status::{})", self.status)
        };
        let display = if Show::is_default(&self.display) {
            String::default()
        } else {
            format!("\t.display(Show::{})", self.display)
        };
        let version = if Version::is_default(&self.version) {
            String::default()
        } else {
            let s: &'static str = self.version.into();
            format!("\t.version(Version::{})", s)
        };
        let optional = if self.optional { "\t.optional()" } else { "" }.to_string();
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            let tags: String = self
                .tags
                .iter()
                .map(|tag| format!("{:?}", tag))
                .intersperse_with(|| ", ".to_string())
                .collect();
            format!("\t.tags([{tags}])")
        };

        let metadata: String = [
            label,
            version_label,
            explainer,
            subtitle,
            language,
            source,
            alternate_sources,
            status,
            display,
            version,
            optional,
            tags,
        ]
        .into_iter()
        .filter(|n| !n.is_empty())
        .intersperse_with(|| "\n".to_string())
        .collect();

        let content = self.content.to_rust_code(start_tabs + 1);
        let tabs = (0..start_tabs).map(|_| '\t').collect::<String>();

        if metadata.is_empty() {
            format!("{tabs}Document::from({content}){condition}")
        } else {
            format!("{tabs}Document::new(){condition}{metadata}\n\t.content({content})")
        }
        .replace('\n', &format!("\n{tabs}"))
    }
}

impl ToRustCode for Content {
    fn to_rust_code(&self, start_tabs: usize) -> String {
        match &self {
            Content::Liturgy(content) => {
                let children: String = content
                    .body
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Liturgy::from(vec![\n{children}\n])")
            }
            Content::Series(content) => {
                let children: String = content
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Series::from(vec![\n{children}\n])")
            }
            Content::Choice(content) => {
                let children: String = content
                    .options
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                let selected = if content.selected == 0 {
                    String::new()
                } else {
                    format!(".selected({})", content.selected)
                };
                let should_rotate = if content.should_rotate {
                    ".should_rotate()"
                } else {
                    ""
                };
                format!("Choice::from(vec![\n{children}\n]){selected}{should_rotate}")
            }
            Content::Parallel(content) => {
                let children: String = content
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Parallel::from(vec![\n{children}\n])")
            }
            Content::Text(content) => {
                let response = if let Some(response) = &content.response {
                    format!(r#".response("{response}")"#)
                } else {
                    String::new()
                };
                let display_format = if content.display_format.is_default() {
                    String::new()
                } else {
                    format!(".display_format(DisplayFormat::{})", content.display_format)
                };
                format!(
                    r#"Text::from({:?}){}{}"#,
                    content.text, response, display_format
                )
            }
            Content::Heading(content) => match content {
                Heading::InsertDate => "Heading::InsertDate".to_string(),
                Heading::InsertDay => "Heading::InsertDay".to_string(),
                Heading::Date(s) => format!("Heading::Date({:?})", s),
                Heading::Day {
                    name,
                    proper,
                    holy_days,
                } => format!(
                    "Heading::Day {{ name: {:?}, proper: {:?}, holy_days: {:?} }}",
                    name, proper, holy_days
                ),
                Heading::Text(level, text) => {
                    format!("Heading::from((HeadingLevel::{}, {:?}))", level, text)
                }
            },
            Content::Antiphon(content) => format!("Antiphon::from({:?})", content),
            Content::Litany(content) => {
                let children = content
                    .iter()
                    .map(|line| format!("{:?}", line))
                    .intersperse_with(|| ",\n\t\t".to_string())
                    .collect::<String>();
                format!(
                    "Litany::from((\n\t{:?},\n\t[\n\t\t{}\n\t]\n))",
                    content.response, children
                )
            }
            Content::ResponsivePrayer(content) => {
                let children = content
                    .iter()
                    .map(|line| format!("{:?}", line))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("ResponsivePrayer::from([\n\t{}\n])", children)
            }
            Content::Rubric(content) => {
                let long = if content.long { ".long()" } else { "" };
                format!("Rubric::from({:?}){}", content.text, long)
            }
            Content::CollectOfTheDay { allow_multiple } => format!(
                "Content::CollectOfTheDay {{ allow_multiple: {} }}",
                allow_multiple
            ),
            Content::Empty => "Content::Empty".to_string(),
            Content::Error(content) => {
                format!("Content::Error(DocumentError::from({:?}))", content)
            }
            Content::BiblicalCitation(content) => {
                let intro = content
                    .intro
                    .as_ref()
                    .map(|intro| {
                        let doc: Document = intro.clone().into();
                        format!(
                            ".intro(BiblicalReadingIntro::from({}))",
                            doc.to_rust_code(start_tabs)
                        )
                    })
                    .unwrap_or_default();
                format!("BiblicalCitation::from({:?}){}", content.citation, intro)
            }
            Content::BiblicalReading(_) => String::new(),
            Content::Canticle(_) => String::new(),
            Content::CanticleTableEntry(_) => String::new(),
            Content::DocumentLink { .. } => String::new(),
            Content::GloriaPatri(content) => {
                let display_format = if content.display_format.is_default() {
                    String::new()
                } else {
                    format!(".display_format(DisplayFormat::{})", content.display_format)
                };
                format!(
                    "GloriaPatri::from({:?}, {:?}, {:?}, {:?}){}",
                    content.text.0, content.text.1, content.text.2, content.text.3, display_format
                )
            }
            Content::HymnLink(_) => String::new(),
            Content::Invitatory(_) => String::new(),
            Content::LectionaryReading(content) => {
                let reading_type = match &content.reading_type {
                    ReadingTypeTable::Preference(key) => {
                        format!("ReadingTypeTable::Preference({})", key.to_rust_code(0))
                    }
                    ReadingTypeTable::Selected(reading_type) => {
                        format!("ReadingTypeTable::Selected(ReadingType::{reading_type})")
                    }
                };
                let override_ty = match &content.reading_type_overridden_by {
                    None => "None".into(),
                    Some(reading_type) => format!("Some(ReadingType::{reading_type})"),
                };
                let lectionary = match &content.lectionary {
                    LectionaryTableChoice::Preference(key) => {
                        format!("LectionaryTableChoice::Preference({})", key.to_rust_code(0))
                    }
                    LectionaryTableChoice::Selected(lectionary) => {
                        format!("LectionaryTableChoice::Selected({lectionary})")
                    }
                };
                let intro = match &content.intro {
                    None => "None".into(),
                    Some(template) => {
                        let doc: Document = template.clone().into();
                        format!(
                            "Some(BiblicalReadingIntroTemplate::from({}))",
                            doc.to_rust_code(0)
                        )
                    }
                };
                format!("LectionaryReading {{\n\treading_type: {reading_type},\n\treading_type_overridden_by: {override_ty},\n\tlectionary: {lectionary},\n\tintro: {intro}\n}}")
            }
            Content::Preces(content) => {
                let children = content
                    .iter()
                    .map(|(v, r)| format!("({:?}, {:?})", v, r))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("Preces::from([\n\t{}\n])", children)
            }
            Content::Psalm(_) => String::new(),
            Content::PsalmCitation(_) => String::new(),
            Content::Sentence(content) => {
                let citation = content
                    .citation
                    .as_ref()
                    .map(|citation| format!(".citation({:?})", citation))
                    .unwrap_or_default();
                format!("Sentence::from({:?}){}", content.text, citation)
            }
        }
    }
}

impl ToRustCode for Condition {
    #![allow(clippy::tabs_in_doc_comments)]
    /// Converts a [Condition](liturgy::Condition) into a `String` consisting of compileable Rust code
    /// ```
    /// # use liturgy::Condition;
    /// # use calendar::Season;
    /// # use crate::to_rust_code::ToRustCode;
    /// let cond = Condition::Any(vec![
    ///     Condition::Season(Season::Easter),
    ///     Condition::Season(Season::Ascension),
    ///     Condition::Season(Season::Pentecost)
    /// ]);
    /// let cond_str = r#"Condition::Any(vec![
    /// 	Condition::Season(Season::Easter),
    /// 	Condition::Season(Season::Ascension),
    /// 	Condition::Season(Season::Pentecost)
    /// ])"#;
    /// assert_eq!(&cond.to_rust_code(0), cond_str)
    /// ```
    fn to_rust_code(&self, start_tabs: usize) -> String {
        let c = match self {
            Condition::Day(day) => format!("Day({})", day.to_rust_code(0)),
            Condition::Feast(feast) => format!("Feast(Feast::{})", feast),
            Condition::Season(season) => format!("Season(Season::{})", season),
            Condition::ObservedSeason(season) => format!("ObservedSeason(Season::{})", season),
            Condition::Week(week) => format!("Week(LiturgicalWeek::{})", week),
            Condition::Weekday(weekday) => format!("Weekday(Weekday::{})", weekday),
            Condition::Evening => "Evening".to_string(),
            Condition::RankGte(rank) => format!("RankGte(Rank::{rank})"),
            Condition::DateLt(m, d) => format!("DateLt({m}, {d})"),
            Condition::DateLte(m, d) => format!("DateLte({m}, {d})"),
            Condition::DateGt(m, d) => format!("DateGt({m}, {d})"),
            Condition::DateGte(m, d) => format!("DateGte({m}, {d})"),
            Condition::DayOfMonth(d) => format!("DayOfMonth({d})"),
            Condition::Preference(key, value) => format!(
                "Preference({}, {})",
                key.to_rust_code(0),
                value.to_rust_code(0)
            ),
            Condition::Not(cond) => format!("Not(Box::new({}))", cond.to_rust_code(0)),
            Condition::And(a, b) => format!(
                "And(Box::new({}), Box::new({}))",
                a.to_rust_code(0),
                b.to_rust_code(0)
            ),
            Condition::Or(a, b) => format!(
                "Or(Box::new({}), Box::new({}))",
                a.to_rust_code(0),
                b.to_rust_code(0)
            ),
            Condition::Any(conds) => {
                let conds = conds
                    .iter()
                    .map(|cond| cond.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("Any(vec![\n\t{conds}\n])")
            }
            Condition::All(conds) => {
                let conds = conds
                    .iter()
                    .map(|cond| cond.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("All(vec![\n\t{conds}\n])")
            }
            Condition::None(conds) => {
                let conds = conds
                    .iter()
                    .map(|cond| cond.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("None(vec![\n\t{conds}\n])")
            }
        };
        format!("Condition::{c}")
    }
}

impl ToRustCode for LiturgicalDayId {
    fn to_rust_code(&self, _start_tabs: usize) -> String {
        match self {
            LiturgicalDayId::Feast(feast) => format!("LiturgicalDayId::Feast(Feast::{feast})"),
            LiturgicalDayId::WeekAndDay(w, d) => {
                format!("LiturgicalDayId::WeekAndDay(LiturgicalWeek::{w}, Weekday::{d})")
            }
            LiturgicalDayId::ProperAndDay(p, d) => {
                format!("LiturgicalDayId::ProperAndDay(Proper::{p}, Weekday::{d})")
            }
            LiturgicalDayId::TransferredFeast(feast) => {
                format!("LiturgicalDayId::TransferredFeast(Feast::{feast})")
            }
            LiturgicalDayId::DayOfMonth(d) => format!("LiturgicalDayId::DayOfMonth({d})"),
            LiturgicalDayId::VariousOccasions(o) => {
                format!("LiturgicalDayId::VariousOccasions(VariousOccasions::{o})")
            }
        }
    }
}

impl ToRustCode for PreferenceKey {
    fn to_rust_code(&self, _start_tabs: usize) -> String {
        match self {
            PreferenceKey::Global(g) => {
                let s: &'static str = g.into();
                format!("PreferenceKey::Global(GlobalPref::{s})")
            }
            PreferenceKey::Local(l) => format!("PreferenceKey::Local({:?}.to_string())", l),
        }
    }
}

impl ToRustCode for PreferenceValue {
    fn to_rust_code(&self, _start_tabs: usize) -> String {
        match self {
            PreferenceValue::Language(val) => format!("PreferenceValue::Language({val})"),
            PreferenceValue::Version(val) => format!("PreferenceValue::Version({val})"),
            PreferenceValue::Lectionary(val) => format!("PreferenceValue::Lectionary({val})"),
            PreferenceValue::CanticleTable(val) => format!("PreferenceValue::CanticleTable({val})"),
            PreferenceValue::ReadingType(val) => format!("PreferenceValue::ReadingType({val})"),
            PreferenceValue::Local(val) => format!("PreferenceValue::Local({:?}.to_string())", val),
            PreferenceValue::Bool(val) => format!("PreferenceValue::Bool({val})"),
            PreferenceValue::Canticle(val) => {
                format!("PreferenceValue::Canticle({val})")
            }
        }
    }
}
