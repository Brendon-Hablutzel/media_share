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

    fn get_file_abspath(&self, label: &str) -> String {
        self.files_directory
            .join(&label)
            .to_str()
            .expect("file path should be valid unicode")
            .to_owned()
    }

    pub async fn store(&self, data: Bytes, label: &str) -> Result<(), std::io::Error> {
        let file_location = self.get_file_abspath(label);

        let mut file = tokio::fs::File::create(&file_location).await?;
        file.write(&data).await?;

        Ok(())
    }

    pub async fn get(&self, label: &str) -> Result<ReaderStream<tokio::fs::File>, std::io::Error> {
        let file_location = self.get_file_abspath(label);

        let file = tokio::fs::File::open(&file_location).await?;

        Ok(ReaderStream::new(file))
    }

    pub async fn remove(&self, label: &str) -> Result<(), std::io::Error> {
        let file_location = self.get_file_abspath(label);

        tokio::fs::remove_file(&file_location).await
    }
}
