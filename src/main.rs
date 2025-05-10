use askama::Template;
use axum::response::IntoResponse;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use std::{env, io, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// Configuration for the application
struct AppConfig {
    jar_directory: PathBuf,
    jar_filename: String,
}

// Template for the index page
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    jar_filename: String,
}

#[tokio::main]
async fn main() {
    // Configuration
    let jar_directory = env::var("JAR_DIRECTORY").unwrap_or_else(|_| "./jars".to_string());
    let jar_filename = env::var("JAR_FILENAME").unwrap_or_else(|_| "application.jar".to_string());

    let config = Arc::new(AppConfig {
        jar_directory: PathBuf::from(jar_directory),
        jar_filename,
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/download/{filename}", get(download_handler))
        .with_state(config);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the index page
async fn index_handler(State(config): State<Arc<AppConfig>>) -> impl IntoResponse {
    IndexTemplate {
        jar_filename: config.jar_filename.clone(),
    }
    .render()
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
