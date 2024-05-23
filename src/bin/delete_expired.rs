use dotenv::dotenv;
use media_share::record_store::PgStore;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let uri =
        std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL should be set");

    let record_store = PgStore::new(&uri)
        .await
        .expect("should be able to initialize the data store");

    record_store.delete_expired().await.unwrap();
}
