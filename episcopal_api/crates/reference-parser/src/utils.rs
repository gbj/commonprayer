use crate::{query::BibleVersePart, BibleReferenceQuery, BibleReferenceRange, Book};

const SINGLE_CHAPTER_BOOKS: [Book; 5] = [
    Book::Obadiah,
    Book::SecondJohn,
    Book::ThirdJohn,
    Book::Philemon,
    Book::Jude,
];
const POSSIBLE_BRACKET_DELIMITERS: [&str; 7] = ["", ",", ";", "[", "]", "(", ")"];
const VERSE_CITATION_CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

#[cfg(any(feature = "browser", feature = "regex"))]
pub fn parse_reference(reference: &str) -> Vec<BibleReferenceRange> {
    let mut list: Vec<BibleReferenceRange> = Vec::new();
    let mut prev: Option<BibleReferenceRange> = None;
    let mut bracket_opened = false;

    // basic case -- add a range for each of the pieces of the citation
    for part in split_str_and_keep_delimiters(reference, &[',', ';', '[', ']', '(', ')'][..]) {
        let trimmed = part.trim();
        // if it's only a delimiter, open or close bracket if necessary, but otherwise do nothing
        if POSSIBLE_BRACKET_DELIMITERS
            .iter()
            .any(|delimiter| *delimiter == trimmed)
        {
            if trimmed == "[" || trimmed == "(" {
                bracket_opened = true;
            } else if trimmed == "]" || trimmed == ")" {
                bracket_opened = false;
            }
        } else {
            let current = parse_single_reference(
                &part,
                prev.or_else(|| list.last().cloned()),
                bracket_opened,
            );
            list.push(current);
            prev = Some(current);
        }
    }

    // handle citations like 1 Cor. 13:[1-3]4-13
    let starts_with_bracket = list
        .get(0)
        .map(|range| range.start)
        .and_then(|range| range.verse)
        .is_none()
        && list.get(1).map(|range| range.bracketed).unwrap_or(false);

    if starts_with_bracket {
        let start_book = fallback_to_previous_entry(&list, |range| range.start.book);
        let start_chapter = fallback_to_previous_entry(&list, |range| range.start.chapter);
        let start_verse = fallback_to_previous_entry(&list, |range| range.start.verse);

        let end_book =
            fallback_to_previous_entry(&list, |range| range.end.and_then(|query| query.book))
                .or(start_book);

        let end_chapter =
            fallback_to_previous_entry(&list, |range| range.end.and_then(|query| query.chapter))
                .or(start_chapter);

        let end_verse =
            fallback_to_previous_entry(&list, |range| range.end.and_then(|query| query.verse))
                .or(start_verse);

        list.remove(0);
        list[0] = BibleReferenceRange {
            start: BibleReferenceQuery {
                book: start_book,
                chapter: start_chapter,
                verse: start_verse,
                verse_part: BibleVersePart::All,
            },
            end: Some(BibleReferenceQuery {
                book: end_book,
                chapter: end_chapter,
                verse: end_verse,
                verse_part: BibleVersePart::All,
            }),
            bracketed: true,
        };
    }

    // chapter list like Psalms 120, 121, 122
    // without this, it would interpret that as 120:1, (120:121), (120:122)
    let first_chapter_has_verse = list
        .get(0)
        .map(|range| range.start.verse.is_some())
        .unwrap_or(false);
    if !first_chapter_has_verse {
        for range in list.iter_mut().skip(1) {
            if range.start.chapter.is_none() && range.start.verse.is_some() {
                range.start.chapter = range.start.verse.take();
            } else {
                break;
            }
        }
    }

    // return the list
    list
}

fn fallback_to_previous_entry<T>(
    list: &[BibleReferenceRange],
    field: fn(&BibleReferenceRange) -> Option<T>,
) -> Option<T> {
    match (list.get(1).and_then(field), list.get(0).and_then(field)) {
        (Some(v), Some(_)) => Some(v),
        (None, Some(v)) => Some(v),
        (Some(v), None) => Some(v),
        (None, None) => None,
    }
}

fn split_str_and_keep_delimiters(text: &str, delimiters: &[char]) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(delimiters) {
        if last != index {
            result.push(text[last..index].to_string());
        }
        result.push(matched.to_string());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(text[last..].to_string());
    }
    result
}

#[cfg(any(feature = "browser", feature = "regex"))]
fn parse_single_reference(
    reference: &str,
    previous: Option<BibleReferenceRange>,
    bracketed: bool,
) -> BibleReferenceRange {
    let mut range_pieces = reference.split('-').flat_map(|s| s.split('–'));
    let first_half = range_pieces.next();
    let second_half = range_pieces.next();

    let start_partial_structure = previous.is_some();

    let mut start: BibleReferenceQuery = match first_half {
        Some(cite) => match query_from_re(cite, false, start_partial_structure, None) {
            Some(query) => {
                BibleReferenceQuery {
                    book: query.book.or_else(|| {
                        previous.and_then(|prev| {
                            let prev_end = prev.end.and_then(|end| end.book);
                            let prev_start = prev.start.book;
                            prev_end.or(prev_start)
                        })
                    }),
                    chapter: query.chapter.or_else(|| {
                        // only use previous chapter if previous citation had both chapter and verse
                        // so this won't, for example, think that Psalm 120, 121 is Psalm 120, Psalm 120:121
                        previous.and_then(|prev| {
                            if prev.start.verse.is_some() || bracketed {
                                let prev_end = prev.end.and_then(|end| end.chapter);
                                let prev_start = prev.start.chapter;
                                prev_end.or(prev_start)
                            } else {
                                None
                            }
                        })
                    }),
                    ..query
                }
            }
            None => {
                return BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: None,
                        verse_part: BibleVersePart::All,
                    },
                    end: None,
                    bracketed,
                };
            }
        },
        None => {
            return BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: None,
                    chapter: None,
                    verse: None,
                    verse_part: BibleVersePart::All,
                },
                end: None,
                bracketed,
            };
        }
    };

    if start.book.is_none() {
        if let Some(book) = previous.and_then(|range| range.start.book) {
            start.book = Some(book);
        }
    }

    // fill out the start of the range with the details of the end of the previous range
    // e.g., 1 Tim. 4:1-3, 4-6 will fill out with 1 Tim., chapter 4
    let previous_end: Option<BibleReferenceQuery> = match previous {
        Some(range) => range.end,
        None => None,
    };
    let augmented_start = fill_out(Some(start), previous_end);

    let end = match second_half {
        Some(cite) => query_from_re(cite, true, true, augmented_start),
        None => None,
    };

    BibleReferenceRange {
        start: match augmented_start {
            Some(augmented) => augmented,
            None => start,
        },
        end,
        bracketed,
    }
}

#[cfg(feature = "regex")]
fn match_first_half(reference: &str) -> Option<[Option<String>; 4]> {
    lazy_static::lazy_static! {
        static ref FIRST_HALF_RE: regex::Regex =
            regex::Regex::new(r#"([\d\s]*[\w\.]+[a-zA-Z\s]*)\s*(\d+)?:?(\d+)?"#)
                .expect("could not compile Regex");
    }
    // TODO if we compile these statically it's probably faster
    let captures = FIRST_HALF_RE.captures(reference.trim())?;
    let mut iter = captures.iter();
    Some([
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
    ])
}

#[cfg(feature = "regex")]
fn match_second_half(reference: &str) -> Option<[Option<String>; 4]> {
    lazy_static::lazy_static! {
        static ref SECOND_HALF_RE: regex::Regex =
            regex::Regex::new(r"([\d\s]*[\w\.]+)\s*(\d+)?:?(\d+)?")
                .expect("could not compile Regex");
    }
    // TODO if we compile these statically it's probably faster
    let captures = SECOND_HALF_RE.captures(reference.trim())?;
    let mut iter = captures.iter();
    Some([
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
        iter.next().unwrap().map(|m| m.as_str().to_string()),
    ])
}

#[cfg(all(feature = "browser", not(feature = "regex")))]
fn match_first_half(reference: &str) -> Option<[Option<String>; 4]> {
    let re = js_sys::RegExp::new(r#"([\d\s]*[\w\.]+[a-zA-Z\s]*)\s*(\d+)?:?(\d+)?"#, "");
    let results = re.exec(reference);
    results.map(|res| {
        [
            res.get(0).as_string(),
            res.get(1).as_string(),
            res.get(2).as_string(),
            res.get(3).as_string(),
        ]
    })
}

#[cfg(all(feature = "browser", not(feature = "regex")))]
fn match_second_half(reference: &str) -> Option<[Option<String>; 4]> {
    let re = js_sys::RegExp::new(r#"([\d\s]*[\w\.]+)\s*(\d+)?:?(\d+)?"#, "");
    let results = re.exec(reference);
    results.map(|res| {
        [
            res.get(0).as_string(),
            res.get(1).as_string(),
            res.get(2).as_string(),
            res.get(3).as_string(),
        ]
    })
}

#[cfg(any(feature = "browser", feature = "regex"))]
fn query_from_re(
    reference: &str,
    second_half: bool,
    partial_structure: bool,
    template: Option<BibleReferenceQuery>,
) -> Option<BibleReferenceQuery> {
    let matches = if second_half {
        match_second_half(reference)
    } else {
        match_first_half(reference)
    }?;
    let query: Option<BibleReferenceQuery>;

    if partial_structure {
        // build query based on matches
        // if only one part of Regex matches, assume it's a verse; if two, it's a chapter-verse; if three, book-chapter-verse
        // this allows a match on e.g., [Matthew 1:4-]3 to read "3" as a verse, not as a book name like "3 John"
        query = match matches {
            [Some(_), Some(book_name), Some(chapter_str), Some(verse_str)] => {
                Some(BibleReferenceQuery {
                    book: Some(Book::from(book_name)),
                    chapter: match_to_int(&chapter_str).0,
                    verse: match_to_int(&verse_str).0,
                    verse_part: match_to_int(&verse_str).1,
                })
            }
            [Some(_), Some(chapter_str), None, Some(verse_str)] => Some(BibleReferenceQuery {
                book: None,
                chapter: match_to_int(&chapter_str).0,
                verse: match_to_int(&verse_str).0,
                verse_part: match_to_int(&verse_str).1,
            }),
            [Some(_), Some(chapter_str), Some(verse_str), None] => Some(BibleReferenceQuery {
                book: None,
                chapter: match_to_int(&chapter_str).0,
                verse: match_to_int(&verse_str).0,
                verse_part: match_to_int(&verse_str).1,
            }),
            [Some(_), Some(verse_str), None, None] => Some(BibleReferenceQuery {
                book: None,
                chapter: None,
                verse: match_to_int(&verse_str).0,
                verse_part: match_to_int(&verse_str).1,
            }),
            _ => None,
        };
    } else {
        let book = matches[1].as_ref().map(Book::from);
        let mut chapter = match &matches[2] {
            Some(num) => match_to_int(&num).0,
            None => None,
        };
        let mut verse = match &matches[3] {
            Some(num) => match_to_int(&num).0,
            None => None,
        };

        if let Some(book) = book {
            if SINGLE_CHAPTER_BOOKS.iter().any(|b| *b == book)
                && chapter.is_some()
                && verse.is_none()
            {
                verse = chapter;
                chapter = None;
            }
        }

        query = Some(BibleReferenceQuery {
            book,
            chapter,
            verse,
            verse_part: BibleVersePart::All,
        });
    }

    fill_out(query, template)
}

fn fill_out(
    query: Option<BibleReferenceQuery>,
    template: Option<BibleReferenceQuery>,
) -> Option<BibleReferenceQuery> {
    let mut final_query: Option<BibleReferenceQuery> = query;

    // if template provided, fill out query as needed
    if let Some(tpl) = template {
        if let Some(mut q) = query {
            if q.book.is_none() {
                q.book = tpl.book;
            }
            if q.chapter.is_none() {
                q.chapter = tpl.chapter;
            }
            if q.verse.is_none() {
                q.verse = tpl.verse;
            }
            final_query = Some(q);
        }
    }

    final_query
}

fn match_to_int(input_str: &str) -> (Option<u16>, BibleVersePart) {
    let input_digits_only = input_str.replace(|c| VERSE_CITATION_CHARS.contains(&c), "");
    let verse_number = match str::parse::<u16>(&input_digits_only) {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let bible_verse_part = if input_str.contains('a') {
        BibleVersePart::A
    } else if input_str.contains('b') {
        BibleVersePart::B
    } else if input_str.contains('c') {
        BibleVersePart::C
    } else if input_str.contains('d') {
        BibleVersePart::D
    } else {
        BibleVersePart::All
    };
    (verse_number, bible_verse_part)
}
