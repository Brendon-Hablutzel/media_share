use crate::errors::AppError;
use axum::body::Bytes;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

#[derive(Clone)]
pub struct FilesystemStore {
    files_directory: PathBuf,
}

impl FilesystemStore {
    pub fn new(files_directory: &std::path::Path) -> Self {
        Self {
            files_directory: files_directory.to_owned(),
        }
    }

    pub async fn store(
        &self,
        data: Bytes,
        label: &str,
        file_extension: Option<&str>,
    ) -> Result<String, AppError> {
        // takes a file and a label, stores the file using the label,
        // and returns the location of the file
        let file_extension = match file_extension {
            Some(extension) => format!(".{extension}"),
            None => String::new(),
        };

        let file_name = format!("{label}{file_extension}");

        let abs_file_path = self
            .files_directory
            .join(&file_name)
            .to_str()
            .expect("file path should be valid unicode")
            .to_owned();

        let mut file = tokio::fs::File::create(&abs_file_path).await?;
        file.write(&data).await?;

        Ok(abs_file_path)
    }

    pub async fn get(
        &self,
        file_location: &str,
    ) -> Result<ReaderStream<tokio::fs::File>, AppError> {
        let file = tokio::fs::File::open(file_location)
            .await
            .map_err(|_| AppError::NotFound(format!("file not found: {file_location}")))?;

        Ok(ReaderStream::new(file))
    }
}
