use liturgy::{
    BiblicalCitation, Choice, Content, Document, Heading, HeadingLevel, Liturgy, Reference, Series,
    Source, Version,
};

pub const TITLE: &str = "TITLE";
pub const OPENING_RUBRICS: &str = "OPENING_RUBRICS";
pub const WORD_OF_GOD_HEADING: &str = "WORD_OF_GOD_HEADING";
pub const PROCESSION_RUBRIC: &str = "PROCESSION_RUBRIC";
pub const PROCESSION_MUSIC_RUBRIC: &str = "PROCESSION_MUSIC_RUBRIC";
pub const OPENING_HYMN: &str = "OPENING_HYMN";
pub const OPENING_ACCLAMATION: &str = "OPENING_ACCLAMATION";
pub const OPENING_ADDRESS_RUBRIC: &str = "OPENING_ADDRESS_RUBRIC";
pub const OPENING_ADDRESS: &str = "OPENING_ADDRESS";
pub const OPENING_WORDS_TO_COUPLE_RUBRIC: &str = "OPENING_WORDS_TO_COUPLE_RUBRIC";
pub const OPENING_WORDS_TO_COUPLE: &str = "OPENING_WORDS_TO_COUPLE";
pub const CONSENT_HEADER: &str = "CONSENT_HEADER";
pub const CONSENT: &str = "CONSENT";
pub const CONSENT_CONGREGATION: &str = "CONSENT_CONGREGATION";
pub const PRESENTATION_RUBRIC: &str = "PRESENTATION_RUBRIC";
pub const PRESENTATION: &str = "PRESENTATION";
pub const ADDITIONAL_DIRECTIONS_PARALLEL: &str = "ADDITIONAL_DIRECTIONS_PARALLEL";
pub const HYMN_PSALM_OR_ANTHEM: &str = "HYMN_PSALM_OR_ANTHEM";
pub const MINISTRY_OF_THE_WORD: &str = "MINISTRY_OF_THE_WORD";
pub const SALUTATION: &str = "SALUTATION";
pub const COLLECT: &str = "COLLECT";
pub const SCRIPTURE_RUBRIC: &str = "SCRIPTURE_RUBRIC";
pub const FIRST_LESSON: &str = "FIRST_LESSON";
pub const PSALM: &str = "PSALM";
pub const SECOND_LESSON: &str = "SECOND_LESSON";
pub const GOSPEL: &str = "GOSPEL";
pub const HOMILY: &str = "HOMILY";
pub const THE_MARRIAGE_HEADER: &str = "THE_MARRIAGE_HEADER";
pub const THE_MARRIAGE: &str = "THE_MARRIAGE";
pub const THE_RINGS: &str = "THE_RINGS";
pub const RINGS_ALREADY_GIVEN: &str = "RINGS_ALREADY_GIVEN";
pub const PRONOUNCEMENT_HEADER: &str = "PRONOUNCEMENT_HEADER";
pub const PRONOUNCEMENT: &str = "PRONOUNCEMENT";
pub const THE_PRAYERS_HEADER: &str = "THE_PRAYERS_HEADER";
pub const LORDS_PRAYER: &str = "LORDS_PRAYER";
pub const THE_PRAYERS: &str = "THE_PRAYERS";
pub const BLESSING_OF_THE_MARRIAGE: &str = "BLESSING_OF_THE_MARRIAGE";
pub const BLESSING_RUBRIC: &str = "BLESSING_RUBRIC";
pub const BLESSING_PRAYERS: &str = "BLESSING_PRAYERS";
pub const BLESSING_PROPER: &str = "BLESSING_PROPER";
pub const PEACE_HEADER: &str = "PEACE_HEADER";
pub const THE_PEACE: &str = "THE_PEACE";
pub const POST_PEACE_RUBRICS: &str = "POST_PEACE_RUBRICS";
pub const POST_PEACE_HYMN: &str = "POST_PEACE_HYMN";
pub const AT_THE_EUCHARIST: &str = "AT_THE_EUCHARIST";
pub const OFFERTORY_RUBRIC: &str = "OFFERTORY_RUBRIC";
pub const PROPER_PREFACE: &str = "PROPER_PREFACE";
pub const POST_PREFACE_RUBRIC: &str = "POST_PREFACE_RUBRIC";
pub const POSTCOMMUNION_PRAYER: &str = "POSTCOMMUNION_PRAYER";
pub const CLOSING_HYMN: &str = "CLOSING_HYMN";

pub const MARRIAGE_PARALLEL_TAGS: [&str; 50] = [
    TITLE,
    OPENING_RUBRICS,
    WORD_OF_GOD_HEADING,
    PROCESSION_RUBRIC,
    PROCESSION_MUSIC_RUBRIC,
    OPENING_HYMN,
    OPENING_ACCLAMATION,
    OPENING_ADDRESS_RUBRIC,
    OPENING_ADDRESS,
    OPENING_WORDS_TO_COUPLE_RUBRIC,
    OPENING_WORDS_TO_COUPLE,
    CONSENT_HEADER,
    CONSENT,
    CONSENT_CONGREGATION,
    PRESENTATION_RUBRIC,
    PRESENTATION,
    ADDITIONAL_DIRECTIONS_PARALLEL,
    HYMN_PSALM_OR_ANTHEM,
    MINISTRY_OF_THE_WORD,
    SALUTATION,
    COLLECT,
    SCRIPTURE_RUBRIC,
    FIRST_LESSON,
    PSALM,
    SECOND_LESSON,
    GOSPEL,
    HOMILY,
    THE_MARRIAGE_HEADER,
    THE_MARRIAGE,
    THE_RINGS,
    RINGS_ALREADY_GIVEN,
    PRONOUNCEMENT_HEADER,
    PRONOUNCEMENT,
    THE_PRAYERS_HEADER,
    LORDS_PRAYER,
    THE_PRAYERS,
    BLESSING_OF_THE_MARRIAGE,
    BLESSING_RUBRIC,
    BLESSING_PRAYERS,
    BLESSING_PROPER,
    PEACE_HEADER,
    THE_PEACE,
    POST_PEACE_RUBRICS,
    POST_PEACE_HYMN,
    AT_THE_EUCHARIST,
    OFFERTORY_RUBRIC,
    PROPER_PREFACE,
    POST_PREFACE_RUBRIC,
    POSTCOMMUNION_PRAYER,
    CLOSING_HYMN,
];

use psalter::bcp1979::*;

lazy_static! {
    pub static ref PARALLEL_READINGS: Document = Document::new()
        .label("Wedding Readings")
        .version(Version::Parallel)
        .page(426)
        .alternate_source(Reference {
            source: Source::LiturgicalResources2,
            page: 18
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "Wedding Readings"))),
            Document::from(Heading::from((HeadingLevel::Heading2, "The First Lesson"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Genesis 1:26-28")),
                Document::from(BiblicalCitation::from("Genesis 2:4-9, 15-24")),
                Document::from(BiblicalCitation::from("Song of Solomon 2:10-13; 8:6-7")),
                Document::from(BiblicalCitation::from("Tobit 8:5b-8")),
                Document::from(BiblicalCitation::from("Ruth 1:16–17")),
                Document::from(BiblicalCitation::from("1 Samuel 18:1b, 3; 20:16–17; 42a")),
                Document::from(BiblicalCitation::from("1 Samuel 18:1-4")),
                Document::from(BiblicalCitation::from("Ecclesiastes 4:9-12")),
                Document::from(BiblicalCitation::from("Song of Solomon 2:10–13; 8:6–7")),
                Document::from(BiblicalCitation::from("Micah 4:1-4")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Psalm"))),
            Document::from(Choice::from(vec![
                Document::from(PSALM_65.clone()),
                Document::from(PSALM_67.clone()),
                Document::from(PSALM_85.clone().citation("Psalm 85:7-13")),
                Document::from(PSALM_98.clone()),
                Document::from(PSALM_100.clone()),
                Document::from(PSALM_126.clone()),
                Document::from(PSALM_127.clone()),
                Document::from(PSALM_128.clone()),
                Document::from(PSALM_133.clone()),
                Document::from(PSALM_148.clone()),
                Document::from(PSALM_149.clone().citation("Psalm 149:1-5")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Second Lesson"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("1 Corinthians 13:1-13")),
                Document::from(BiblicalCitation::from("1 Corinthians 12:31b–13:13")),
                Document::from(BiblicalCitation::from("Ephesians 3:14-19")),
                Document::from(BiblicalCitation::from("Ephesians 3:14–21")),
                Document::from(BiblicalCitation::from("Ephesians 5:1-2, 21-33")),
                Document::from(BiblicalCitation::from("Colossians 3:12-17")),
                Document::from(BiblicalCitation::from("1 John 4:7-16")),
                Document::from(BiblicalCitation::from("Romans 12:9–18")),
                Document::from(BiblicalCitation::from("2 Corinthians 5:17–20")),
                Document::from(BiblicalCitation::from("Galatians 5:14, 22–26")),
                Document::from(BiblicalCitation::from("1 John 3:18–24")),
                Document::from(BiblicalCitation::from("1 John 4:7–16, 21")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Gospel"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Matthew 5:1-10")),
                Document::from(BiblicalCitation::from("Matthew 5:13-16")),
                Document::from(BiblicalCitation::from("Matthew 5:1-16")),
                Document::from(BiblicalCitation::from("Matthew 7:21, 24-29")),
                Document::from(BiblicalCitation::from("Mark 10:6-9, 13-16")),
                Document::from(BiblicalCitation::from("Mark 12:28–34")),
                Document::from(BiblicalCitation::from("Luke 6:32-38")),
                Document::from(BiblicalCitation::from("John 15:9-12")),
                Document::from(BiblicalCitation::from("John 15:9–17")),
                Document::from(BiblicalCitation::from("John 17:1–2, 18–26")),
            ]))
        ]))));
}
