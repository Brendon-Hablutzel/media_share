use crate::MediaRecord;
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

    pub async fn insert(&self, media: MediaRecord) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO media (label, content_type, expiry) VALUES ($1, $2, $3)",
            &media.label,
            &media.content_type,
            &media.expiry
        )
        .execute(&self.connection_pool)
        .await?;

        Ok(())
    }

    pub async fn get_one(&self, label: &str) -> Result<Option<MediaRecord>, sqlx::Error> {
        sqlx::query_as!(MediaRecord, "SELECT * FROM media WHERE label = $1", label)
            .fetch_optional(&self.connection_pool)
            .await
    }

    pub async fn delete_expired(&self) -> Result<impl Iterator<Item = MediaRecord>, sqlx::Error> {
        let current_time = chrono::offset::Utc::now();

        let records = sqlx::query_as!(
            MediaRecord,
            "DELETE FROM media WHERE expiry <= $1 RETURNING *",
            current_time
        )
        .fetch_all(&self.connection_pool)
        .await?;

        Ok(records.into_iter())
    }
}
