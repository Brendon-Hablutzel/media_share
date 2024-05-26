use dotenv::dotenv;
use std::path::{Path, PathBuf};

pub struct Config {
    database_url: String,
    files_dir: PathBuf,
    expiry_time: chrono::Duration,
    port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL should be set");

        let files_dir =
            std::env::var("FILES_DIR").expect("environment variable FILES_DIR should be set");
        let files_dir = std::path::Path::new(&files_dir).to_owned();

        let expiry_hours =
            std::env::var("EXPIRY_HOURS").expect("environment variable EXPIRY_HOURS should be set");
        let expiry_hours = expiry_hours
            .parse::<u16>()
            .expect("EXPIRY_HOURS should be a valid non-negative integer");
        let expiry_time = chrono::Duration::hours(expiry_hours.into());

        let port = std::env::var("PORT").expect("environment variable PORT should be set");

        Self {
            database_url,
            files_dir,
            expiry_time,
            port,
        }
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }

    pub fn get_files_dir(&self) -> &Path {
        &self.files_dir
    }

    pub fn get_expiry_time(&self) -> chrono::Duration {
        self.expiry_time
    }

    pub fn get_port(&self) -> &str {
        &self.port
    }
}
