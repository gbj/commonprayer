use std::fmt::Display;

use crate::{Content, Document};

pub fn build_parallel_table<T, U>(
    parallel_tags: T,
    docs: &[&Document],
) -> Vec<Vec<(Document, usize)>>
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    let mut parallels: Vec<Vec<(Document, usize)>> = Vec::new();

    for tag in parallel_tags.into_iter() {
        let mut parallel_tagged_docs = Vec::new();

        // chunk each parallel doc and add them
        for doc in docs {
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
            let mut parallels_for_this_row: Vec<(Document, usize)> = Vec::new();

            for (column_id, column) in row.iter().enumerate() {
                let prev_child = if column_id == 0 {
                    None
                } else {
                    row.get(column_id - 1)
                };
                if prev_child.is_none() || prev_child.unwrap().content != column.content {
                    let mut width = 1;
                    for subsequent_idx in (column_id + 1)..row.len() {
                        let subsequent_doc = row.get(subsequent_idx);
                        if subsequent_doc.is_some()
                            && subsequent_doc.unwrap().content == column.content
                        {
                            width += 1;
                        } else {
                            break;
                        }
                    }
                    parallels_for_this_row.push((column.clone(), width));
                }
            }

            parallels.push(parallels_for_this_row);
        }
    }

    parallels
}
