use crate::book_abbrevs::BOOKS;
use language::Language;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Book {
    // OT books
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    // Apocrypha
    Tobit,
    Judith,
    Ester,
    Wisdom,
    Ecclesiasticus,
    Baruch,
    EpistleJeremiah,
    PrayerOfAzariah,
    Susanna,
    FirstMaccabees,
    SecondMaccabees,
    FirstEsdras,
    SecondEsdras,
    FourthEsdras,
    Psalm151,
    ThirdMaccabees,
    FourthMaccabees,
    Bel,
    // NT books
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
    None,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let book_name = BOOKS
            .iter()
            .find(|(_, s_book)| s_book == self)
            .map(|(name, _)| *name);
        write!(f, "{}", book_name.unwrap_or_default())
    }
}

impl From<&str> for Book {
    fn from(book_name: &str) -> Self {
        let ratings = BOOKS
            .iter()
            .map(|(abbrev, book)| (strsim::sorensen_dice(book_name, abbrev), book));

        let (_, closest_book) = ratings
            .max_by(|(rating_a, _), (rating_b, _)| {
                rating_a.partial_cmp(rating_b).unwrap_or(Ordering::Equal)
            })
            .unwrap_or((0.0, &Book::None));

        *closest_book
    }
}

impl From<String> for Book {
    fn from(book_name: String) -> Self {
        Self::from(book_name.as_str())
    }
}

impl From<&String> for Book {
    fn from(book_name: &String) -> Self {
        Self::from(book_name.as_str())
    }
}

impl Book {
    pub fn book_short_name(&self, _language: Language) -> &'static str {
        // TODO other languages
        match self {
            Book::Genesis => "Genesis",
            Book::Exodus => "Exodus",
            Book::Leviticus => "Leviticus",
            Book::Numbers => "Numbers",
            Book::Deuteronomy => "Deuteronomy",
            Book::Joshua => "Joshua",
            Book::Judges => "Judges",
            Book::Ruth => "Ruth",
            Book::FirstSamuel => "1 Samuel",
            Book::SecondSamuel => "2 Samuel",
            Book::FirstKings => "1 Kings",
            Book::SecondKings => "2 Kings",
            Book::FirstChronicles => "1 Chronicles",
            Book::SecondChronicles => "2 Chronicles",
            Book::Ezra => "Ezra",
            Book::Nehemiah => "Nehemiah",
            Book::Esther => "Esther",
            Book::Job => "Job",
            Book::Psalms => "Psalms",
            Book::Proverbs => "Proverbs",
            Book::Ecclesiastes => "Ecclesiastes",
            Book::SongOfSolomon => "The Song of Solomon",
            Book::Isaiah => "Isaiah",
            Book::Jeremiah => "Jeremiah",
            Book::Lamentations => "Lamentations",
            Book::Ezekiel => "Ezekiel",
            Book::Daniel => "Daniel",
            Book::Hosea => "Hosea",
            Book::Joel => "Joel",
            Book::Amos => "Amos",
            Book::Obadiah => "Obadiah",
            Book::Jonah => "Jonah",
            Book::Micah => "Micah",
            Book::Nahum => "Nahum",
            Book::Habakkuk => "Habakkuk",
            Book::Zephaniah => "Zephaniah",
            Book::Haggai => "Haggai",
            Book::Zechariah => "Zechariah",
            Book::Malachi => "Malachi",
            Book::Tobit => "Tobit",
            Book::PrayerOfAzariah => "The Song of the Three Children",
            Book::Judith => "Judith",
            Book::Baruch => "Baruch",
            Book::FirstMaccabees => "1 Maccabees",
            Book::SecondMaccabees => "2 Maccabees",
            Book::Wisdom => "Wisdom",
            Book::Ecclesiasticus => "Sirach",
            Book::Matthew => "Matthew",
            Book::Mark => "Mark",
            Book::Luke => "Luke",
            Book::John => "John",
            Book::Acts => "Acts",
            Book::Romans => "Romans",
            Book::FirstCorinthians => "1 Corinthians",
            Book::SecondCorinthians => "2 Corinthians",
            Book::Galatians => "Galatians",
            Book::Ephesians => "Ephesians",
            Book::Philippians => "Philippians",
            Book::Colossians => "Colossians",
            Book::FirstThessalonians => "1 Thessalonians",
            Book::SecondThessalonians => "2 Thessalonians",
            Book::FirstTimothy => "1 Timothy",
            Book::SecondTimothy => "2 Timothy",
            Book::Titus => "Titus",
            Book::Philemon => "Philemon",
            Book::Hebrews => "Hebrews",
            Book::James => "James",
            Book::FirstPeter => "1 Peter",
            Book::SecondPeter => "2 Peter",
            Book::FirstJohn => "1 John",
            Book::SecondJohn => "2 John",
            Book::ThirdJohn => "3 John",
            Book::Jude => "Jude",
            Book::Revelation => "Revelation",
            Book::FirstEsdras => "1 Esdras",
            Book::SecondEsdras => "2 Esdras",
            Book::Bel => "Bel",
            Book::EpistleJeremiah => "Letter of Jeremiah",
            Book::Ester => "Esther",
            Book::Susanna => "Susanna",
            Book::FourthEsdras => "4 Esdras",
            Book::Psalm151 => "Psalm 151",
            Book::ThirdMaccabees => "3 Maccabees",
            Book::FourthMaccabees => "4 Maccabees",
            Book::None => "",
        }
    }

    pub fn book_long_name(&self, _language: Language) -> &'static str {
        match self {
            Book::Genesis => "The Book of Genesis",
            Book::Exodus => "The Book of Exodus",
            Book::Leviticus => "The Book of Leviticus",
            Book::Numbers => "The Book of Numbers",
            Book::Deuteronomy => "The Book of Deuteronomy",
            Book::Joshua => "The Book of Joshua",
            Book::Judges => "The Book of Judges",
            Book::Ruth => "The Book of Ruth",
            Book::FirstSamuel => "The First Book of Samuel",
            Book::SecondSamuel => "The Second Book of Samuel",
            Book::FirstKings => "The First Book of Kings",
            Book::SecondKings => "The Second Book of Kings",
            Book::FirstChronicles => "The First Book of Chronicles",
            Book::SecondChronicles => "The Second Book of Chronicles",
            Book::Ezra => "The Book of Ezra",
            Book::Nehemiah => "The Book of Nehemiah",
            Book::Esther => "The Book of Esther",
            Book::Job => "The Book of Job",
            Book::Psalms => "The Psalms",
            Book::Proverbs => "The Book of Proverbs",
            Book::Ecclesiastes => "The Book of Ecclesiastes",
            Book::SongOfSolomon => "The Song of Solomon",
            Book::Isaiah => "The Book of the Prophet Isaiah",
            Book::Jeremiah => "The Book of the Prophet Jeremiah",
            Book::Lamentations => "The Book of Lamentations",
            Book::Ezekiel => "The Book of the Prophet Ezekiel",
            Book::Daniel => "The Book of Daniel",
            Book::Hosea => "The Book of the Prophet Hosea",
            Book::Joel => "The Book of the Prophet Joel",
            Book::Amos => "The Book of the Prophet Amos",
            Book::Obadiah => "The Book of the Prophet Obadiah",
            Book::Jonah => "The Book of the Prophet Jonah",
            Book::Micah => "The Book of the Prophet Micah",
            Book::Nahum => "The Book of the Prophet Nahum",
            Book::Habakkuk => "The Book of the Prophet Habakkuk",
            Book::Zephaniah => "The Book of the Prophet Zephaniah",
            Book::Haggai => "The Book of the Prophet Haggai",
            Book::Zechariah => "The Book of the Prophet Zechariah",
            Book::Malachi => "The Book of the Prophet Malachi",
            Book::Tobit => "The Book of Tobit",
            Book::PrayerOfAzariah => "The Song of the Three Children",
            Book::Judith => "The Book of Judith",
            Book::Baruch => "The Book of Baruch",
            Book::FirstMaccabees => "The First Book of Maccabees",
            Book::SecondMaccabees => "The Second Book of Maccabees",
            Book::Wisdom => "The Wisdom of Solomon",
            Book::Ecclesiasticus => "The Wisdom of Ben Sira",
            Book::Matthew => "The Gospel According to Matthew",
            Book::Mark => "The Gospel According to Mark",
            Book::Luke => "The Gospel According to Luke",
            Book::John => "The Gospel According to John",
            Book::Acts => "The Acts of the Apostles",
            Book::Romans => "The Letter to the Romans",
            Book::FirstCorinthians => "The First Letter to the Corinthians",
            Book::SecondCorinthians => "The Second Letter to the Corinthians",
            Book::Galatians => "The Letter to the Galatians",
            Book::Ephesians => "The Letter to the Ephesians",
            Book::Philippians => "The Letter to the Philippians",
            Book::Colossians => "The Letter to the Colossians",
            Book::FirstThessalonians => "The First Letter to the Thessalonians",
            Book::SecondThessalonians => "The Second Letter to the Thessalonians",
            Book::FirstTimothy => "The First Letter to Timothy",
            Book::SecondTimothy => "The Second Letter to Timothy",
            Book::Titus => "The Letter to Titus",
            Book::Philemon => "The Letter to Philemon",
            Book::Hebrews => "The Book of Hebrews",
            Book::James => "The Letter of James",
            Book::FirstPeter => "The First Letter of Peter",
            Book::SecondPeter => "The Second Letter of Peter",
            Book::FirstJohn => "The First Letter of John",
            Book::SecondJohn => "The Second Letter of John",
            Book::ThirdJohn => "The Third Letter of John",
            Book::Jude => "The Letter of Jude",
            Book::Revelation => "The Book of Revelation",
            Book::FirstEsdras => "The Second Book of Esdras",
            Book::SecondEsdras => "The Second Book of Esdras",
            Book::Bel => "Bel and the Dragon",
            Book::EpistleJeremiah => "The Letter of Jeremiah",
            Book::Ester => "The Book of Esther",
            Book::Susanna => "Susanna",
            Book::FourthEsdras => "The Fourth Book of Esdras",
            Book::Psalm151 => "Psalm 151",
            Book::ThirdMaccabees => "The Third Book of Maccabees",
            Book::FourthMaccabees => "The Fourth Book of Maccabees",
            Book::None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Book;

    #[test]
    fn exact_matches() {
        assert_eq!(Book::from("1 Cor."), Book::FirstCorinthians);
        assert_eq!(Book::from("Matt."), Book::Matthew);
        assert_eq!(Book::from("John"), Book::John);
        assert_eq!(Book::from("Neh."), Book::Nehemiah);
    }

    #[test]
    fn misspellings() {
        assert_eq!(Book::from("1 Tin."), Book::FirstTimothy);
    }

    #[test]
    fn possible_ambiguities() {
        assert_eq!(Book::from("Ecclus."), Book::Ecclesiasticus);
        assert_eq!(Book::from("Eccl."), Book::Ecclesiastes);
        assert_eq!(Book::from("1 Ch"), Book::FirstChronicles);
        assert_eq!(Book::from("Phil"), Book::Philippians);
    }
}
