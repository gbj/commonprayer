use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Content, Document, Reference, SlugPath};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ParallelDocument {
    Source(Option<Reference>),
    Link { label: String, slug: SlugPath },
    Explainer(Option<String>),
    Document(Box<Document>),
}

pub fn build_parallel_table<T, U>(
    parallel_tags: T,
    docs: &[(&SlugPath, &Document)],
) -> Vec<Vec<(ParallelDocument, usize)>>
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    let mut parallels: Vec<Vec<(ParallelDocument, usize)>> = Vec::new();

    // push source links
    parallels.push(
        docs.iter()
            .map(|(_, doc)| (ParallelDocument::Source(doc.source), 1))
            .collect(),
    );

    // push URLs
    parallels.push(
        docs.iter()
            .map(|(slug, doc)| {
                (
                    ParallelDocument::Link {
                        label: doc.label.clone().unwrap_or_default(),
                        slug: (*slug).clone(),
                    },
                    1,
                )
            })
            .collect(),
    );

    // push explainers
    if docs.iter().any(|(_, doc)| doc.explainer.is_some()) {
        parallels.push(
            docs.iter()
                .map(|(_, doc)| (ParallelDocument::Explainer(doc.explainer.clone()), 1))
                .collect(),
        );
    }

    // push contents of documents
    for tag in parallel_tags.into_iter() {
        let mut parallel_tagged_docs = Vec::new();

        // chunk each parallel doc and add them
        for (_, doc) in docs {
            let children_with_this_tag_in_this_doc = doc.children_with_tag(tag.to_string());

            parallel_tagged_docs.push(
                children_with_this_tag_in_this_doc
                    .flat_map(|child| child.clone().to_smallest_chunks())
                    .collect::<Vec<_>>(),
            );
        }

        // rearrange from a list of columns (by doc) to a list of rows (by chunk)
        let mut chunked_rows = Vec::new();

        let max_len = parallel_tagged_docs
            .iter()
            .map(|column| column.len())
            .max()
            .unwrap_or(0);
        for row_idx in 0..max_len {
            chunked_rows.push(
                parallel_tagged_docs
                    .iter()
                    .map(move |column| {
                        column
                            .get(row_idx)
                            .cloned()
                            .unwrap_or_else(|| Document::from(Content::Empty))
                    })
                    .collect::<Vec<_>>(),
            );
        }

        // deduplicate/expand width of columns
        for row in chunked_rows {
            let mut parallels_for_this_row: Vec<(ParallelDocument, usize)> = Vec::new();

            for (column_id, column) in row.iter().enumerate() {
                let prev_child = if column_id == 0 {
                    None
                } else {
                    row.get(column_id - 1)
                };
                if prev_child.is_none()
                    || prev_child.unwrap().content != column.content
                    || prev_child.unwrap().version != column.version
                {
                    let mut width = 1;
                    for subsequent_idx in (column_id + 1)..row.len() {
                        let subsequent_doc = row.get(subsequent_idx);
                        if subsequent_doc.is_some()
                            && subsequent_doc.unwrap().content == column.content
                            && subsequent_doc.unwrap().version == column.version
                        {
                            width += 1;
                        } else {
                            break;
                        }
                    }
                    parallels_for_this_row
                        .push((ParallelDocument::Document(Box::new(column.clone())), width));
                }
            }

            parallels.push(parallels_for_this_row);
        }
    }

    parallels
}
