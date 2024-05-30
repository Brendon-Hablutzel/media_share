use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, Multipart},
};
use chrono::{self, DateTime, Utc};
pub mod record_store;
use record_store::PgStore;
use sqlx::prelude::FromRow;
pub mod config;
pub mod errors;
pub mod file_store;
pub mod templates;

// represents a media file record in the database
#[derive(FromRow)]
pub struct MediaRecord {
    pub label: String,
    pub content_type: String,
    pub expiry: DateTime<Utc>,
}

// represents a file sent to the server by the upload form
#[derive(Debug)]
pub struct MultipartFile {
    pub name: String,
    pub content_type: String, // this should be a HTTP header compatible type
    pub data: Bytes,
}

pub async fn get_multipart_file_by_name(
    multipart: &mut Multipart,
    target_name: &str,
) -> Result<Option<MultipartFile>, MultipartError> {
    // take a stream of multipart data and return the file matching
    // the given name, if such a file exists
    let mut data = None;

    while let Some(field) = multipart.next_field().await? {
        let Some(field_name) = field.name() else {
            continue;
        };

        if field_name == target_name {
            let Some(file_name) = field.file_name() else {
                continue;
            };
            let file_name = file_name.to_owned();

            let Some(content_type) = field.content_type() else {
                continue;
            };
            let content_type = content_type.to_owned();

            let bytes = field.bytes().await?;
            data = Some(MultipartFile {
                name: file_name,
                content_type,
                data: bytes,
            });
        }
    }

    Ok(data)
}

pub async fn insert_with_unique_label(
    record_store: &PgStore,
    content_type: &str,
    expiry: chrono::DateTime<Utc>,
) -> Result<String, sqlx::Error> {
    let word_1 = random_word::gen(random_word::Lang::En);
    let word_2 = random_word::gen(random_word::Lang::En);
    let mut counter: u32 = 0;

    loop {
        let counter_string = match counter {
            0 => String::new(),
            nonzero => format!("-{nonzero}"),
        };

        let label = format!("{word_1}-{word_2}{counter_string}");

        let media_record = MediaRecord {
            label: label.clone(),
            content_type: content_type.to_owned(),
            expiry,
        };

        match record_store.insert(media_record).await {
            Ok(_) => return Ok(label),
            Err(sqlx_err) => {
                if let sqlx::Error::Database(db_err) = &sqlx_err {
                    if !db_err.is_unique_violation() {
                        return Err(sqlx_err);
                    }
                }
            }
        }

        counter += 1;
    }
}
