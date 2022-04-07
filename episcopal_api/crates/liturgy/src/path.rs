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

    pub fn remove_at_path(&mut self, path: &[usize]) -> Result<Document, PathError> {
        // first, get all but last item of path
        let mut path = path.to_vec();
        let idx = path.pop();
        let parent = self.at_path_mut(path)?;
        if let Some(idx) = idx {
            match &mut parent.content {
                Content::Liturgy(ref mut content) => Ok(content.body.remove_at_index(idx)),
                Content::Series(ref mut content) => Ok(content.remove_at_index(idx)),
                Content::Parallel(ref mut content) => Ok(content.remove_at_index(idx)),
                Content::Choice(ref mut content) => Ok(content.remove_at_index(idx)),
                _ => Err(PathError::DoesNotExist),
            }
        } else {
            Err(PathError::DoesNotExist)
        }
    }

    pub fn move_subdocument(
        &mut self,
        start_path: &[usize],
        end_path: &[usize],
    ) -> Result<(), PathError> {
        let old_doc = self.remove_at_path(start_path)?;
        let mut end_path = end_path.to_vec();
        let idx = end_path.pop().ok_or(PathError::DoesNotExist)?;
        let parent = self.at_path_mut(end_path)?;
        parent.insert_at(idx, old_doc)?;
        Ok(())
    }

    pub fn insert_at(&mut self, index: usize, doc: Document) -> Result<(), PathError> {
        match &mut self.content {
            Content::Liturgy(c) => {
                c.body.insert_at(index, doc);
                Ok(())
            }
            Content::Series(c) => {
                c.insert_at(index, doc);
                Ok(())
            }
            Content::Choice(c) => {
                c.insert_at(index, doc);
                Ok(())
            }
            Content::Parallel(c) => {
                c.insert_at(index, doc);
                Ok(())
            }
            _ => Err(PathError::DoesNotExist),
        }
    }
}
