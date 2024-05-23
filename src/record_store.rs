use crate::Media;
use sqlx;
use sqlx::Row;

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
        let query = "INSERT INTO media (label, file_location, content_type, expiry) VALUES ($1, $2, $3, $4)";

        sqlx::query(query)
            .bind(&media.label)
            .bind(&media.file_location)
            .bind(&media.content_type)
            .bind(&media.expiry)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }

    pub async fn get_one(&self, label: &str) -> Result<Option<Media>, sqlx::Error> {
        let query = "SELECT * FROM media WHERE label = $1";

        let result = sqlx::query(query)
            .bind(label)
            .fetch_optional(&self.connection_pool)
            .await?;

        let media = result.map(|row| Media {
            label: row.get("label"),
            file_location: row.get("file_location"),
            content_type: row.get("content_type"),
            expiry: row.get("expiry"),
        });

        Ok(media)
    }

    pub async fn delete_expired(&self) -> Result<(), sqlx::Error> {
        let current_time = chrono::offset::Utc::now();

        let query = "DELETE FROM media WHERE expiry <= $1";

        sqlx::query(query)
            .bind(current_time)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }
}
