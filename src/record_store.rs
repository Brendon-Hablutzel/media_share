use crate::Media;
use sqlx;

#[derive(Clone)]
pub struct PgStore {
    connection_pool: sqlx::postgres::PgPool,
}

impl PgStore {
    pub async fn new(uri: &str) -> Result<Self, sqlx::Error> {
        let connection_pool = sqlx::postgres::PgPool::connect(&uri).await?;

        Ok(Self { connection_pool })
    }

    pub async fn insert(&self, media: Media) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO media (label, file_location, content_type, expiry) VALUES ($1, $2, $3, $4)",
            &media.label,
            &media.file_location,
            &media.content_type,
            &media.expiry
        )
        .execute(&self.connection_pool)
        .await?;

        Ok(())
    }

    pub async fn get_one(&self, label: &str) -> Result<Option<Media>, sqlx::Error> {
        let result = sqlx::query!("SELECT * FROM media WHERE label = $1", label)
            .fetch_optional(&self.connection_pool)
            .await?;

        let media = result.map(|record| Media {
            label: record.label,
            file_location: record.file_location,
            content_type: record.content_type,
            expiry: record.expiry,
        });

        Ok(media)
    }

    pub async fn delete_expired(&self) -> Result<impl Iterator<Item = Media>, sqlx::Error> {
        let current_time = chrono::offset::Utc::now();

        let result = sqlx::query!(
            "DELETE FROM media WHERE expiry <= $1 RETURNING *",
            current_time
        )
        .fetch_all(&self.connection_pool)
        .await?;

        let media = result.into_iter().map(|record| Media {
            label: record.label,
            file_location: record.file_location,
            content_type: record.content_type,
            expiry: record.expiry,
        });

        Ok(media)
    }
}
