use dotenv::dotenv;
use media_share::{errors::AppError, file_store::FilesystemStore, record_store::PgStore};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let uri =
        std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL should be set");

    let files_dir =
        std::env::var("FILES_DIR").expect("environment variable FILES_DIR should be set");

    let record_store = PgStore::new(&uri).await?;

    let file_store = FilesystemStore::new(std::path::Path::new(&files_dir));

    let deleted = record_store.delete_expired().await?;

    for file in deleted {
        file_store.remove(&file.file_location).await?;
    }

    Ok(())
}
