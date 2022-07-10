use docx::DocxDocument;
use futures::future::join_all;
use liturgy::{Content, Document, DocumentError};
use std::{fs::File, path::PathBuf};
use tempfile::tempdir;
use thiserror::Error;

use super::reading_loader::ReadingLoader;

// DOCX export
pub fn docx_response(
    file_name: impl std::fmt::Display,
    docx: DocxDocument,
) -> Result<PathBuf, DocxExportError> {
    let file_name = format!("{}.docx", file_name);

    let dir = tempdir().map_err(|_| DocxExportError::TempDir)?;
    let path = dir.path().join(file_name);
    eprintln!("path = {:?}", path);
    let file = File::create(&path).map_err(|_| DocxExportError::CreateFile)?;

    docx.write(&file).map_err(|_| DocxExportError::WriteFile)?;

    // TODO this is a silly way to create a temp dir that will live long enough for server to return response
    Box::leak(Box::new(dir));

    Ok(path)
}

pub async fn add_readings(docx: DocxDocument, readings: Vec<ReadingLoader>) -> DocxDocument {
    let readings = join_all(readings.into_iter().map(ReadingLoader::into_future)).await;
    readings
        .into_iter()
        .fold(docx, |docx, reading| match reading {
            Ok(reading) => docx.add_content(&Document::from(reading)),
            Err(e) => docx.add_content(&Document::from(Content::Error(DocumentError::from(
                e.to_string(),
            )))),
        })
}

#[derive(Error, Debug)]
pub enum DocxExportError {
    #[error("could not create temporary directory")]
    TempDir,
    #[error("could not create DOCX file")]
    CreateFile,
    #[error("could not write to DOCX file")]
    WriteFile,
}
