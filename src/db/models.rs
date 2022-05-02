use chrono::Utc;
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct ApiKey {
    pub id: i32,
    pub key_owner: String,
    pub apikey: String,
    pub expires_at: chrono::DateTime<Utc>,
}

impl ApiKey {
    /// Returns `Some(ApiKey)` if the key exists and is valid, otherwise `None`
    pub async fn get_key<'a>(
        key: &'a str,
        pool: &Pool<Postgres>,
    ) -> Result<Option<ApiKey>, sqlx::Error> {
        let result = sqlx::query_as!(ApiKey, "SELECT * FROM apikeys WHERE apikey = $1", key)
            .fetch_one(pool)
            .await?;

        let expired_timestamp = result.expires_at.timestamp();
        let current_timestamp = chrono::Utc::now().timestamp();

        println!(
            "current: {}, expires_at: {}",
            current_timestamp, expired_timestamp
        );

        if result.apikey == key && (current_timestamp < expired_timestamp) {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub struct File {
    pub id: String,
    pub file_name: String,
    pub file_hash: String,
    pub file_type: i16, // 0=IMAGE, 1=VIDEO, 2=OTHER
    pub file_size: i32,
    pub uploader: String, // as API Key
    pub upload_date: chrono::DateTime<Utc>,
}

impl File {
    /// Inserts new file into database
    pub async fn create(&mut self, pool: &Pool<Postgres>) -> Result<String, sqlx::Error> {
        let result= sqlx::query!(
            "INSERT INTO files(id, file_name, file_hash, file_type, file_size, uploader, upload_date) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id;",
            self.id,
            self.file_name,
            self.file_hash,
            self.file_type,
            self.file_size,
            self.uploader,
            self.upload_date,
        ).fetch_one(pool).await?;

        Ok(result.id)
    }

    /// Gets file by id, returning the file struct if it exists
    pub async fn get_by_id(id: String, pool: &Pool<Postgres>) -> Result<File, sqlx::Error> {
        sqlx::query_as!(File, "SELECT * FROM files WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    /// Returns `id` if a file with the given `hash` exists otherwise `None`
    pub async fn get_id_by_hash(
        hash: String,
        pool: &Pool<Postgres>,
    ) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query!("SELECT id FROM files WHERE file_hash = $1", hash)
            .fetch_optional(pool)
            .await?;

        if let Some(record) = row {
            return Ok(Some(record.id));
        }

        Ok(None)
    }
}
