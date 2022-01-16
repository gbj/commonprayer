use thiserror::Error;

use crate::{Content, Document};

#[derive(Error, Debug)]
pub enum PathError {
    #[error("the specified path does not exist")]
    DoesNotExist,
    #[error("there were remaining segments left in path, but the last document node contains no children")]
    RemainingSegments,
}
impl Document {
    /// Deeply retrieves a sub-document by traversing child [Document]s
    /// that can contain children ([Series], [Liturgy], [Choice], [Parallel])
    /// ```
    /// # use crate::liturgy::*;
    /// let doc = Document::from(Series::from([
    ///   Document::from(Choice::from([
    ///     Document::from(Text::from("0.0")),
    ///     Document::from(Text::from("0.1")),
    ///     Document::from(Text::from("0.2"))
    ///   ])),
    ///   Document::from(Text::from("1"))
    /// ]));
    /// assert!(doc.at_path([0, 1]).is_ok());
    /// assert_eq!(doc.at_path([0, 1]).unwrap(), &Document::from(Text::from("0.1")));
    /// assert!(doc.at_path([1]).is_ok());
    /// assert_eq!(doc.at_path([1]).unwrap(), &Document::from(Text::from("1")));
    /// ```
    pub fn at_path(&self, path: impl IntoIterator<Item = usize>) -> Result<&Document, PathError> {
        self.path_helper(path.into_iter())
    }

    fn path_helper(&self, mut path: impl Iterator<Item = usize>) -> Result<&Document, PathError> {
        let idx = path.next();
        match idx {
            // path iterator is exhausted, return this document
            None => Ok(self),
            Some(idx) => {
                let children = match &self.content {
                    Content::Liturgy(doc) => Some(doc.body.as_slice()),
                    Content::Series(doc) => Some(doc.as_slice()),
                    Content::Parallel(doc) => Some(doc.as_slice()),
                    Content::Choice(doc) => Some(doc.options.as_slice()),
                    _ => None,
                };
                match children {
                    None => Err(PathError::RemainingSegments),
                    Some(children) => {
                        let indexed_child = children.get(idx);
                        match indexed_child {
                            None => Err(PathError::DoesNotExist),
                            Some(child) => child.path_helper(path),
                        }
                    }
                }
            }
        }
    }

    pub fn at_path_mut(
        &mut self,
        path: impl IntoIterator<Item = usize>,
    ) -> Result<&mut Document, PathError> {
        self.path_helper_mut(path.into_iter())
    }

    fn path_helper_mut(
        &mut self,
        mut path: impl Iterator<Item = usize>,
    ) -> Result<&mut Document, PathError> {
        let idx = path.next();
        match idx {
            // path iterator is exhausted, return this document
            None => Ok(self),
            Some(idx) => {
                let children = match &mut self.content {
                    Content::Liturgy(doc) => Some(doc.body.as_mut_slice()),
                    Content::Series(doc) => Some(doc.as_mut_slice()),
                    Content::Parallel(doc) => Some(doc.as_mut_slice()),
                    Content::Choice(doc) => Some(doc.options.as_mut_slice()),
                    _ => None,
                };
                match children {
                    None => Err(PathError::RemainingSegments),
                    Some(children) => {
                        let indexed_child = children.get_mut(idx);
                        match indexed_child {
                            None => Err(PathError::DoesNotExist),
                            Some(child) => child.path_helper_mut(path),
                        }
                    }
                }
            }
        }
    }
}
