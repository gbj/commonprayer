use itertools::Itertools;
use liturgy::*;
use serde_json::{Map, Value};

struct ConvertableDocument(Document);
pub struct LdfJson(Value);

impl LdfJson {
    pub fn into_inner(self) -> Value {
        self.0
    }
}

impl From<Document> for LdfJson {
    fn from(doc: Document) -> Self {
        let doc = ConvertableDocument(doc);

        // add metadata that's shared across doc types
        let mut map = Map::new();
        map.insert("type".to_string(), doc.as_ldf_type().into_value());
        map.insert("style".to_string(), doc.as_ldf_style().into_value());
        map.insert("label".to_string(), doc.0.label.into_value());

        // TODO add type-specific metadata

        map.insert("value".to_string(), doc.0.content.into_value());
        LdfJson(Value::Object(map))
    }
}

impl ConvertableDocument {
    fn as_ldf_type(&self) -> Option<&'static str> {
        match &self.0.content {
            Content::Series(_) | Content::Liturgy(_) => Some("liturgy"),
            Content::Parallel(_) | Content::Choice(_) => Some("option"),
            Content::Antiphon(_) | Content::GloriaPatri(_) => Some("refrain"),
            Content::BiblicalCitation(_) | Content::Sentence(_) | Content::BiblicalReading(_) => {
                Some("bible-reading")
            }
            Content::Canticle(_) | Content::Invitatory(_) | Content::Psalm(_) => Some("psalm"),
            Content::Heading(_) => Some("heading"),
            Content::Litany(_) | Content::Preces(_) | Content::ResponsivePrayer(_) => {
                Some("responsive")
            }
            Content::Rubric(_) => Some("rubric"),
            Content::Text(_) | Content::CollectOfTheDay { .. } => Some("text"),
            _ => None,
        }
    }

    fn as_ldf_style(&self) -> Option<&'static str> {
        match &self.0.content {
            Content::CollectOfTheDay { .. } => Some("prayer"),
            Content::Antiphon(_) => Some("antiphon"),
            Content::BiblicalCitation(_) => Some("long"),
            Content::BiblicalReading(_) => Some("long"),
            Content::Canticle(_) => Some("canticle"),
            Content::GloriaPatri(_) => Some("gloria"),
            Content::Heading(heading) => match heading {
                Heading::InsertDate => Some("date"),
                Heading::InsertDay => Some("day"),
                Heading::Date(_) => Some("text"),
                Heading::Day { .. } => Some("day"),
                Heading::Text(_, _) => Some("text"),
            },
            Content::Invitatory(_) => Some("psalm"),
            Content::Litany(_) => Some("litany"),
            Content::Preces(_) => Some("preces"),
            Content::Psalm(_) => Some("psalm"),
            Content::ResponsivePrayer(_) => Some("responsive"),
            Content::Sentence(_) => Some("short"),
            Content::Text(_) => Some("text"),
            _ => None,
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl<T> IntoValue for Option<T>
where
    T: std::fmt::Display,
{
    fn into_value(self) -> Value {
        self.map(|s| Value::String(s.to_string()))
            .unwrap_or(Value::Null)
    }
}

impl IntoValue for Content {
    fn into_value(self) -> Value {
        match self {
            Content::Series(c) => c.into_value(),
            Content::Parallel(c) => c.into_value(),
            Content::Choice(c) => c.into_value(),
            Content::Antiphon(c) => c.into_value(),
            Content::BiblicalCitation(_) => Value::Array(vec![]),
            Content::BiblicalReading(c) => c.into_value(),
            Content::Canticle(c) => c.into_value(),
            Content::GloriaPatri(c) => c.into_value(),
            Content::Heading(c) => c.into_value(),
            Content::Invitatory(c) => c.into_value(),
            Content::Litany(c) => c.into_value(),
            Content::Liturgy(c) => c.into_value(),
            Content::Preces(c) => c.into_value(),
            Content::Psalm(c) => c.into_value(),
            Content::ResponsivePrayer(c) => c.into_value(),
            Content::Rubric(c) => c.into_value(),
            Content::Sentence(c) => c.into_value(),
            Content::Text(c) => c.into_value(),
            _ => Value::Null,
        }
    }
}

impl IntoValue for Liturgy {
    fn into_value(self) -> Value {
        self.body.into_value()
    }
}

impl IntoValue for Series {
    fn into_value(self) -> Value {
        Value::Array(
            self.into_vec()
                .into_iter()
                .map(|child| LdfJson::from(child).0)
                .collect(),
        )
    }
}

impl IntoValue for Choice {
    fn into_value(self) -> Value {
        Value::Array(
            self.options
                .into_iter()
                .map(|child| LdfJson::from(child).0)
                .collect(),
        )
    }
}

impl IntoValue for Parallel {
    fn into_value(self) -> Value {
        Value::Array(
            self.into_vec()
                .into_iter()
                .map(|child| LdfJson::from(child).0)
                .collect(),
        )
    }
}

impl IntoValue for Antiphon {
    fn into_value(self) -> Value {
        Value::Array(
            self.to_string()
                .split("\n\n")
                .map(|s| Value::String(String::from(s)))
                .collect(),
        )
    }
}

impl IntoValue for BiblicalReading {
    fn into_value(self) -> Value {
        Value::Array(
            self.text
                .into_iter()
                .map(|(citation, text)| {
                    let mut verse = Map::new();
                    verse.insert("book".to_string(), Value::String(citation.book.to_string()));
                    verse.insert(
                        "chapter".to_string(),
                        Value::String(citation.chapter.to_string()),
                    );
                    verse.insert(
                        "verse".to_string(),
                        Value::String(citation.verse.to_string()),
                    );
                    verse.insert("text".to_string(), Value::String(text));

                    Value::Object(verse)
                })
                .collect(),
        )
    }
}

impl IntoValue for Canticle {
    fn into_value(self) -> Value {
        Value::Array(
            self.sections
                .into_iter()
                .map(|section| {
                    let mut map = Map::new();
                    map.insert(
                        "type".to_string(),
                        Value::String("psalm-section".to_string()),
                    );
                    map.insert("label".to_string(), section.title.into_value());
                    map.insert(
                        "value".to_string(),
                        Value::Array(
                            section
                                .verses
                                .into_iter()
                                .map(|verse| psalm_verse(None as Option<u16>, verse.a, verse.b))
                                .collect(),
                        ),
                    );
                    Value::Object(map)
                })
                .collect(),
        )
    }
}

impl IntoValue for GloriaPatri {
    fn into_value(self) -> Value {
        let (a, b, c, d) = self.text;
        Value::Array(vec![
            Value::String(format!(
                "{} {}&nbsp;*",
                a.replace(' ', "&nbsp;"),
                b.replace(' ', "&nbsp;")
            )),
            Value::String(format!(
                "{} {}",
                c.replace(' ', "&nbsp;"),
                d.replace(' ', "&nbsp;")
            )),
        ])
    }
}

impl IntoValue for Heading {
    fn into_value(self) -> Value {
        match self {
            Heading::InsertDate => Value::Null,
            Heading::InsertDay => Value::Null,
            Heading::Date(s) => Value::Array(vec![Value::String(s)]),
            Heading::Day { name, .. } => Value::Array(vec![Value::String(name)]),
            Heading::Text(_, text) => Value::Array(vec![Value::String(text)]),
        }
    }
}

impl IntoValue for Invitatory {
    fn into_value(self) -> Value {
        Value::Array(
            self.sections
                .into_iter()
                .map(|section| {
                    let mut map = Map::new();
                    map.insert(
                        "type".to_string(),
                        Value::String("psalm-section".to_string()),
                    );
                    map.insert(
                        "value".to_string(),
                        Value::Array(
                            section
                                .verses
                                .into_iter()
                                .map(|verse| psalm_verse(None as Option<u16>, verse.a, verse.b))
                                .collect(),
                        ),
                    );
                    Value::Object(map)
                })
                .collect(),
        )
    }
}

impl IntoValue for Litany {
    fn into_value(self) -> Value {
        Value::Array(
            self.into_vec()
                .into_iter()
                .map(|line| {
                    let mut m = Map::new();
                    m.insert("text".to_string(), Value::from(line));
                    Value::Object(m)
                })
                .collect(),
        )
    }
}

impl IntoValue for Preces {
    fn into_value(self) -> Value {
        Value::Array(
            self.into_vec()
                .into_iter()
                .map(|(label, text)| {
                    let mut m = Map::new();
                    m.insert("label".to_string(), Value::from(label));
                    m.insert("text".to_string(), Value::from(text));
                    Value::Object(m)
                })
                .collect(),
        )
    }
}

impl IntoValue for ResponsivePrayer {
    fn into_value(self) -> Value {
        Value::Array(
            self.into_vec()
                .into_iter()
                .chunks(2)
                .into_iter()
                .map(|mut n| {
                    let mut m = Map::new();
                    let text = n.next().unwrap_or_default();
                    let response = n.next().unwrap_or_default();
                    m.insert("text".to_string(), Value::from(text));
                    m.insert("response".to_string(), Value::from(response));
                    Value::Object(m)
                })
                .collect(),
        )
    }
}

fn psalm_verse<V: std::fmt::Display>(number: Option<V>, a: String, b: String) -> Value {
    let mut map = Map::new();
    map.insert("number".to_string(), number.into_value());
    map.insert("verse".to_string(), Value::String(a));
    map.insert("halfverse".to_string(), Value::String(b));
    Value::Object(map)
}

impl IntoValue for Psalm {
    fn into_value(self) -> Value {
        Value::Array(
            self.sections
                .into_iter()
                .map(|section| {
                    let mut map = Map::new();
                    map.insert(
                        "type".to_string(),
                        Value::String("psalm-section".to_string()),
                    );
                    map.insert("label".to_string(), Value::String(section.local_name));
                    map.insert(
                        "value".to_string(),
                        Value::Array(
                            section
                                .verses
                                .into_iter()
                                .map(|verse| psalm_verse(Some(verse.number), verse.a, verse.b))
                                .collect(),
                        ),
                    );
                    Value::Object(map)
                })
                .collect(),
        )
    }
}

impl IntoValue for Sentence {
    fn into_value(self) -> Value {
        Value::Array(vec![{
            let mut verse = Map::new();
            verse.insert("text".to_string(), Value::String(self.text));
            Value::Object(verse)
        }])
    }
}

impl IntoValue for Text {
    fn into_value(self) -> Value {
        Value::Array(
            self.text
                .split("\n\n")
                .map(|s| Value::String(String::from(s)))
                .collect(),
        )
    }
}

impl IntoValue for Rubric {
    fn into_value(self) -> Value {
        Value::Array(
            self.to_string()
                .split("\n\n")
                .map(|s| Value::String(String::from(s)))
                .collect(),
        )
    }
}
