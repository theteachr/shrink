use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use shrink::{error::Storage, generators::RB62, Shrinker};
use shrink::{shrinkers::Basic, storage::Postgres};
use url::Url;

#[derive(Clone)]
struct AppState {
    main: Arc<RwLock<Basic<RB62, Postgres>>>,
    base_url: Url,
}

impl AppState {
    fn shrink_response(&self, code: &str) -> ShrinkResponse {
        ShrinkResponse {
            shrunk: self.base_url.join(code).unwrap(),
        }
    }
}

#[derive(serde::Serialize)]
struct ShrinkResponse {
    shrunk: Url,
}

#[derive(serde::Deserialize)]
struct ShrinkRequest {
    url: Url,
}

#[derive(serde::Deserialize)]
struct CustomShrinkRequest {
    code: String,
    url: Url,
}

async fn custom_code(
    State(app): State<AppState>,
    body: Json<CustomShrinkRequest>,
) -> Result<Json<ShrinkResponse>, (StatusCode, &'static str)> {
    app.main
        .write()
        .await
        .store_custom(body.url.clone(), &body.code)
        .map_err(|e| match e {
            Storage::Duplicate => (StatusCode::CONFLICT, "code already used"),
            Storage::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        })?;

    Ok(Json(app.shrink_response(&body.code)))
}

async fn shrink(
    State(app): State<AppState>,
    body: Json<ShrinkRequest>,
) -> Result<Json<ShrinkResponse>, (StatusCode, &'static str)> {
    // XXX: Maybe inefficient because of locking the entire database?
    let code = app
        .main
        .write()
        .await
        .shrink(body.url.clone())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "internal error"))?;

    Ok(Json(app.shrink_response(&code)))
}

async fn redirect(
    State(app): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let url = app
        .main
        .read()
        .await
        .expand(&code)
        .map_err(|_| (StatusCode::NOT_FOUND, "shrink code not found"))?;

    // Consider using 302 (Status Found) instead of 307 (Status Temporary Redirect).
    Ok(Redirect::temporary(&url.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Basic::new().await;
    let app = Arc::new(RwLock::new(app));

    let app = AppState {
        main: app,
        base_url: "http://localhost:3000".parse().unwrap(),
    };

    let router = Router::new()
        .route("/", post(shrink).put(custom_code))
        .route("/{code}", get(redirect))
        .with_state(app);

    // TODO: Add a tracing layer.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
