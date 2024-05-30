use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, State},
    http::{header, HeaderMap, HeaderValue},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use media_share::{
    config::Config,
    errors::AppError,
    file_store::FilesystemStore,
    get_multipart_file_by_name, insert_with_unique_label,
    record_store::PgStore,
    templates::{GetFormTemplate, UploadFormTemplate, UploadedResultTemplate},
};

const FILE_UPLOAD_ACTION_NAME: &'static str = "uploadedfile";

#[derive(Clone)]
struct AppState {
    record_store: PgStore,
    file_store: FilesystemStore,
    expiry_time: chrono::Duration,
}

async fn upload_form() -> Html<String> {
    let template = UploadFormTemplate {
        backend_upload_endpoint: "/upload",
        upload_action_name: FILE_UPLOAD_ACTION_NAME,
    };

    Html(template.to_string())
}

async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Html<String>, AppError> {
    let Some(uploaded_file) =
        get_multipart_file_by_name(&mut multipart, FILE_UPLOAD_ACTION_NAME).await?
    else {
        return Err(AppError::NotFound(format!(
            "could not find multipart file for {FILE_UPLOAD_ACTION_NAME}"
        )));
    };

    let expiry = chrono::offset::Utc::now() + state.expiry_time;

    let label =
        insert_with_unique_label(&state.record_store, &uploaded_file.content_type, expiry).await?;

    state.file_store.store(uploaded_file.data, &label).await?;

    let template = UploadedResultTemplate {
        uploaded_file_label: &label,
        expiry_time: &expiry.to_string(),
    };

    Ok(Html(template.to_string()))
}

async fn get_file_form() -> Html<String> {
    let template = GetFormTemplate {};

    Html(template.to_string())
}

async fn get_file(
    State(state): State<AppState>,
    axum::extract::Path(label): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let Some(media) = state.record_store.get_one(&label).await? else {
        return Err(AppError::NotFound(format!("file not found: {label}")));
    };

    let file_stream = state.file_store.get(&media.label).await?;
    let body = Body::from_stream(file_stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&media.content_type)
            .expect("media content type should be a valid http content type header value"),
    );

    Ok((headers, body))
}

async fn create_app(config: &Config) -> Router {
    let record_store = PgStore::new(config.get_database_url())
        .await
        .expect("should be able to initialize the data store");

    let file_store = FilesystemStore::new(config.get_files_dir());

    let state = AppState {
        record_store,
        file_store,
        expiry_time: config.get_expiry_time(),
    };

    Router::new()
        .route("/upload", post(upload))
        .route("/upload", get(upload_form))
        .route("/get/:label", get(get_file))
        .route("/get", get(get_file_form))
        .with_state(state)
        .layer(DefaultBodyLimit::max(1073741824)) // 1 GB
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    let app = create_app(&config).await;

    let host_addr = format!("0.0.0.0:{}", &config.get_port());

    let listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
