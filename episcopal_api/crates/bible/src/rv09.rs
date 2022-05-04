use std::{fs::File, io::Read};

use lazy_static::lazy_static;
use liturgy::Version;
use reference_parser::Book;

use crate::{usx_book_code, OfflineBible, UsxError};

pub struct ReinaValera {}

lazy_static! {
    pub static ref BIBLE_ROOT_DIR: String =
        std::env::var("BIBLE_DIR").unwrap_or_else(|_| String::from("."));
}

impl OfflineBible for ReinaValera {
    fn load_book(book: Book) -> Result<String, UsxError> {
        let code = usx_book_code(book);
        let mut file = File::open(&format!(
            "{}/bibles/RV09/release/USX_1/{}.usx",
            &*BIBLE_ROOT_DIR, code
        ))
        .map_err(|_| UsxError::BookNotFound(book))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|_| UsxError::BookNotFound(book))?;
        Ok(buf)
    }

    fn version() -> liturgy::Version {
        Version::RV09
    }
}

#[cfg(test)]
mod tests {
    use crate::{OfflineBible, ReinaValera};
    use liturgy::{BiblicalReading, Document, Version};
    use reference_parser::{BibleVerse, BibleVersePart, Book};

    #[test]
    fn load_gen_1() {
        assert_eq!(
            ReinaValera::get_citation("Gen. 1:1").unwrap(),
            Document::from(BiblicalReading {
                citation: "Gen. 1:1".to_string(),
                text: vec![(
                    BibleVerse {
                        book: Book::Genesis,
                        chapter: 1,
                        verse: 1,
                        verse_part: BibleVersePart::All
                    },
                    "EN el principio crió Dios los cielos y la tierra.".to_string()
                )],
                intro: None
            })
            .version(Version::RV09)
        );
    }
}
