use liturgy::*;
use psalter::bcp1979::{
    PSALM_106, PSALM_116, PSALM_121, PSALM_130, PSALM_139, PSALM_142, PSALM_23, PSALM_27, PSALM_42,
    PSALM_46, PSALM_90,
};

pub const TITLE: &str = "Title";
pub const OPENING_RUBRIC: &str = "Opening Rubric";
pub const RECEPTION: &str = "Reception";
pub const ANTHEMS: &str = "Anthems";
pub const OPENING_PRAYERS: &str = "Opening Prayers";
pub const LESSONS_HEADING: &str = "Lessons Heading";
pub const LESSONS_RUBRIC: &str = "Lessons Rubric";
pub const READINGS: &str = "Readings";
pub const FIRST_LESSON: &str = "First Lesson";
pub const PSALM_RUBRIC: &str = "Psalm Rubric";
pub const PSALM: &str = "Psalm";
pub const SECOND_LESSON: &str = "Second Lesson";
pub const PSALM_RUBRIC_2: &str = "Psalm Rubric 2";
pub const PSALM_2: &str = "Psalm 2";
pub const GOSPEL_RUBRIC: &str = "Gospel Rubric";
pub const GOSPEL: &str = "Gospel";
pub const HOMILY: &str = "Homily";
pub const CREED: &str = "Apostles’ Creed";
pub const RUBRIC_BEFORE_PRAYERS: &str = "Rubric before Prayers";
pub const PRAYERS_TITLE: &str = "Prayers (Heading)";
pub const PRAYERS_RUBRIC: &str = "Rubric for Prayers";
pub const PRAYERS: &str = "Prayers";
pub const RUBRIC_AFTER_PRAYERS: &str = "Rubric After Prayers";
pub const AT_THE_EUCHARIST_TITLE: &str = "At the Eucharist (Title)";
pub const PROPER_PREFACE: &str = "Proper Preface";
pub const POSTCOMMUNION_PRAYER: &str = "Postcommunion Prayer";
pub const AT_THE_EUCHARIST: &str = "At the Eucharist";
pub const RUBRICS_BEFORE_COMMENDATION: &str = "Rubrics before Commendation";
pub const COMMENDATION: &str = "Commendation";
pub const CLOSING_RUBRIC: &str = "Closing Rubric";
pub const COMMITTAL: &str = "Committal";
pub const COMMITTAL_PRAYERS: &str = "Committal: Prayers";
pub const COMMITTAL_LORDS_PRAYER: &str = "Committal: Lord’s Prayer";
pub const COMMITTAL_PRAYERS_2: &str = "Committal: Prayers 2";
pub const BLESSING: &str = "Blessing";
pub const DISMISSAL: &str = "Dismissal";
pub const CONSECRATION_OF_A_GRAVE: &str = "Consecration of a Grave";
pub const ADDITIONAL_PRAYERS: &str = "Additional Prayers";

pub const BURIAL_PARALLEL_TAGS: [&str; 37] = [
    TITLE,
    OPENING_RUBRIC,
    RECEPTION,
    ANTHEMS,
    OPENING_PRAYERS,
    LESSONS_RUBRIC,
    LESSONS_HEADING,
    FIRST_LESSON,
    PSALM_RUBRIC,
    PSALM,
    SECOND_LESSON,
    PSALM_RUBRIC_2,
    PSALM_2,
    GOSPEL_RUBRIC,
    GOSPEL,
    HOMILY,
    CREED,
    RUBRIC_BEFORE_PRAYERS,
    PRAYERS_TITLE,
    PRAYERS_RUBRIC,
    PRAYERS,
    RUBRIC_AFTER_PRAYERS,
    AT_THE_EUCHARIST_TITLE,
    AT_THE_EUCHARIST,
    PROPER_PREFACE,
    POSTCOMMUNION_PRAYER,
    RUBRICS_BEFORE_COMMENDATION,
    COMMENDATION,
    COMMITTAL,
    COMMITTAL_PRAYERS,
    COMMITTAL_LORDS_PRAYER,
    COMMITTAL_PRAYERS_2,
    BLESSING,
    DISMISSAL,
    CONSECRATION_OF_A_GRAVE,
    ADDITIONAL_PRAYERS,
    CLOSING_RUBRIC,
];

lazy_static! {
    pub static ref PARALLEL_READINGS: Document = Document::new()
        .label("Funeral Readings")
        .version(Version::Parallel)
        .page(494)
        .alternate_source(Reference {
            source: Source::EOW2,
            page: 134
        })
        .alternate_source(Reference {
            source: Source::BookOfOccasionalServices2018,
            page: 188
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "Funeral Readings"))),
            Document::from(Heading::from((HeadingLevel::Heading2, "The First Lesson"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Isaiah 25:6-9")),
                Document::from(BiblicalCitation::from("Isaiah 61:1-3")),
                Document::from(BiblicalCitation::from("Lamentations 3:22-26, 31-33")),
                Document::from(BiblicalCitation::from("Wisdom 3:1-5, 9")),
                Document::from(BiblicalCitation::from("Job 19:21-27a")),
                Document::from(BiblicalCitation::from("Ecclesiastes 3:1-11")),
                Document::from(BiblicalCitation::from("Ecclesiastes 12:1-7")),
                Document::from(BiblicalCitation::from("2 Samuel 12:16-23")),
                Document::from(BiblicalCitation::from("Isaiah 65:17-20, 23-25")),
                Document::from(BiblicalCitation::from("Isaiah 66:7-14")),
                Document::from(BiblicalCitation::from("Jeremiah 31:15-17")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Psalm"))),
            Document::from(Choice::from(vec![
                Document::from(PSALM_23.clone()),
                Document::from(PSALM_27.clone()),
                Document::from(PSALM_42.clone().citation("Psalm 42:1-7")),
                Document::from(PSALM_46.clone()),
                Document::from(PSALM_90.clone()),
                Document::from(PSALM_90.clone().citation("Psalm 90:1-12")),
                Document::from(PSALM_106.clone().citation("Psalm 106:1-5")),
                Document::from(PSALM_116.clone()),
                Document::from(PSALM_121.clone()),
                Document::from(PSALM_130.clone()),
                Document::from(PSALM_139.clone().citation("Psalm 139:1-11")),
                Document::from(PSALM_139.clone().citation("Psalm 139:7-12")),
                Document::from(PSALM_142.clone().citation("Psalm 142:1-6")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Second Lesson"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Romans 8:14-19, 34-35, 37-39")),
                Document::from(BiblicalCitation::from(
                    "1 Corinthians 15:20-26, 35-38, 42-44, 53-58"
                )),
                Document::from(BiblicalCitation::from("2 Corinthians 4:16-5:9")),
                Document::from(BiblicalCitation::from("1 John 3:1-2")),
                Document::from(BiblicalCitation::from("Revelation 7:9-17")),
                Document::from(BiblicalCitation::from("Revelation 21:2-7")),
                Document::from(BiblicalCitation::from("Romans 8:35-39")),
                Document::from(BiblicalCitation::from("Romans 8:31-39")),
                Document::from(BiblicalCitation::from("1 Thessalonians 4:13-14,18")),
                Document::from(BiblicalCitation::from("1 John 3:1-2")),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Gospel"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("John 5:24-27")),
                Document::from(BiblicalCitation::from("John 6:37-40")),
                Document::from(BiblicalCitation::from("John 10:11-16")),
                Document::from(BiblicalCitation::from("John 11:21-27")),
                Document::from(BiblicalCitation::from("John 14:1-6")),
                Document::from(BiblicalCitation::from("Matthew 5:1-10")),
                Document::from(BiblicalCitation::from("Matthew 18:1-5, 10-14")),
                Document::from(BiblicalCitation::from("Mark 10:13-16")),
                Document::from(BiblicalCitation::from("Matthew 19:13-15")),
                Document::from(BiblicalCitation::from("Luke 18:15-17")),
            ])),
        ]))));
}
