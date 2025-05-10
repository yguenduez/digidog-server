use tower_http::services::ServeDir;
use askama::Template;
use axum::response::IntoResponse;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Response, Html},
    routing::get,
    Router,
};
use std::{env, path::PathBuf, sync::Arc};
use axum::body::Body;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;

use tracing_subscriber::prelude::*;
use tracing::info;

// Configuration for the application
struct AppConfig {
    jar_directory: PathBuf,
    video_file: PathBuf,
    jar_filename: String,
    digidog_readme: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    jar_filename: String,
    content: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jar_directory = env::var("JAR_DIRECTORY").unwrap_or_else(|_| "./jars".to_string());
    info!("JAR directory: {}", jar_directory);
    let jar_filename = env::var("JAR_FILENAME").unwrap_or_else(|_| "application.jar".to_string());
    info!("JAR filename: {}", jar_filename);
    let digidog_readme_path = env::var("DIGIDOG_README").unwrap_or_else(|_| "README.md".to_string());
    let digidog_readme_content = markdown::to_html(tokio::fs::read_to_string(digidog_readme_path).await.unwrap().as_str());

    let video_path = env::var("VIDEO_PATH").unwrap_or_else(|_| "video.mp4".to_string());

    let config = Arc::new(AppConfig {
        jar_directory: PathBuf::from(jar_directory),
        video_file: PathBuf::from(video_path),
        jar_filename,
        digidog_readme: digidog_readme_content,
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/download/{filename}", get(download_handler))
        .route("/video", get(serve_video))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(config);

    // Start the server
    info!("Starting server on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the index page
async fn index_handler(State(config): State<Arc<AppConfig>>) -> impl IntoResponse {
    Html(Index {
        jar_filename: config.jar_filename.clone(),
        content: config.digidog_readme.clone(),
    }
    .render().unwrap())
}

async fn serve_video(State(config): State<Arc<AppConfig>>) -> impl IntoResponse {
    // Open the file
    let file =  File::open(&config.video_file).await.unwrap();
    let metadata = tokio::fs::metadata(&config.video_file).await.unwrap();

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Convert the reader into a stream
    let stream = ReaderStream::new(reader);

    // Convert the stream into a StreamBody
    let body = Body::from_stream(stream);

    // Create a response with appropriate headers
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CONTENT_LENGTH, metadata.len())
        .body(body)
        .unwrap()
}

// Handler for file downloads
async fn download_handler(
    State(config): State<Arc<AppConfig>>,
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<Response, StatusCode> {
    // Prevent directory traversal attacks
    if filename.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Ensure the requested file matches our configured JAR file
    if filename != config.jar_filename {
        return Err(StatusCode::NOT_FOUND);
    }

    // Get the file path
    let file_path = config.jar_directory.join(&filename);

    // Read the file
    let mut file = File::open(&file_path).await.map_err(|err| {
        eprintln!("Failed to open file: {}", err);
        StatusCode::NOT_FOUND
    })?;

    // Read file content into memory
    let mut content = Vec::new();
    file.read_to_end(&mut content).await.map_err(|err| {
        eprintln!("Failed to read file: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create a response with appropriate headers
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/java-archive")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(content.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(response)
}
