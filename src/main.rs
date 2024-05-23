use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{header, HeaderMap, HeaderValue},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use media_share::{
    errors::AppError, file_store::FilesystemStore, generate_unique_label,
    get_multipart_file_by_name, record_store::PgStore, templates::UploadFormTemplate, Media,
};

const FILE_UPLOAD_ACTION_NAME: &'static str = "uploadedfile";

#[derive(Clone)]
struct AppState {
    record_store: PgStore,
    file_store: FilesystemStore,
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
) -> Result<String, AppError> {
    let Some(uploaded_file) =
        get_multipart_file_by_name(&mut multipart, FILE_UPLOAD_ACTION_NAME).await?
    else {
        return Err(AppError::NotFound(format!(
            "could not find multipart file from {FILE_UPLOAD_ACTION_NAME}"
        )));
    };

    let label = generate_unique_label(&state.record_store).await?;

    let file_extension = uploaded_file
        .name
        .split_once(".")
        .map(|(_, extension)| extension);

    let file_location = state
        .file_store
        .store(uploaded_file.data, &label, file_extension)
        .await?;

    let media = Media {
        file_location,
        content_type: uploaded_file.content_type,
        label: label.clone(),
        expiry: chrono::offset::Utc::now() + chrono::Duration::hours(1),
    };

    state.record_store.insert(media).await?;

    Ok(label)
}

async fn get_file(
    State(state): State<AppState>,
    axum::extract::Path(label): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let Some(media) = state.record_store.get_one(&label).await? else {
        return Err(AppError::NotFound(format!("file not found: {label}")));
    };

    let file_stream = state.file_store.get(&media.file_location).await?;
    let body = Body::from_stream(file_stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&media.content_type)
            .expect("media content type should be a valid http content type header value"),
    );

    Ok((headers, body))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let uri =
        std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL should be set");
    let files_dir =
        std::env::var("FILES_DIR").expect("environment variable FILES_DIR should be set");

    let record_store = PgStore::new(&uri)
        .await
        .expect("should be able to initialize the data store");

    let file_store = FilesystemStore::new(&files_dir);

    let state = AppState {
        record_store,
        file_store,
    };

    let app = Router::new()
        .route("/upload", post(upload))
        .route("/upload", get(upload_form))
        .route("/get/:label", get(get_file))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
