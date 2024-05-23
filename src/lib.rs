use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, Multipart},
};
use chrono::{self, DateTime, Utc};
pub mod record_store;
use record_store::PgStore;
pub mod errors;
pub mod file_store;
pub mod templates;

// represents a media file record in the database
pub struct Media {
    pub label: String,
    pub file_location: String,
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

pub async fn generate_unique_label(record_store: &PgStore) -> Result<String, sqlx::Error> {
    // try to generate a label using two random english words. if the label
    // exists already, append a counter to the label and keep incrementing that
    // counter until the label is unique
    let word_1 = random_word::gen(random_word::Lang::En);
    let word_2 = random_word::gen(random_word::Lang::En);
    let mut counter: u32 = 0;

    loop {
        // only use the counter if it is nonzero (e.g. at least one media
        // exists with same generated label)
        let counter_string = match counter {
            0 => String::new(),
            nonzero => format!("-{nonzero}"),
        };

        let label = format!("{word_1}-{word_2}{counter_string}");

        // if no media with the above label exists already, then it is unique
        // and can be returned
        if let None = record_store.get_one(&label).await? {
            return Ok(label);
        }

        counter += 1;
    }
}
