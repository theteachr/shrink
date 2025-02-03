use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use shrink::{
    app::App,
    error::{Internal, Load},
    storage::Postgres,
};

use shrink::{error::Storage, generators::RB62, Shrinker};
use url::Url;

async fn shrink(
    State(state): State<AppState>,
    body: Json<ShrinkRequest>,
) -> Result<Json<ShrinkResponse>, Internal> {
    let ShrinkRequest { url } = body.0;
    // XXX: Maybe inefficient because of locking the entire database?
    let code = state.app.write().await.shrink(url)?;
    Ok(Json(state.shrink_response(&code)))
}

async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, Load> {
    let url = state.app.read().await.expand(&code)?;
    // Consider using 302 (Status Found) instead of 307 (Status Temporary Redirect).
    Ok(Redirect::temporary(url.as_str()))
}

async fn custom_code(
    State(state): State<AppState>,
    body: Json<CustomShrinkRequest>,
) -> Result<Json<ShrinkResponse>, Storage> {
    let CustomShrinkRequest { url, code } = body.0;
    state.app.write().await.store_custom(url, &code)?;
    Ok(Json(state.shrink_response(&code)))
}

#[derive(Clone)]
struct AppState {
    app: Arc<RwLock<App<RB62, Postgres>>>,
    base_url: Url, // XXX: Arc this?
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new().await;
    let app = Arc::new(RwLock::new(app));

    let app = AppState {
        app,
        base_url: "http://localhost:3000".parse().unwrap(),
    };

    let router = Router::new()
        .route("/", post(shrink).put(custom_code))
        .route("/{code}", get(redirect))
        .with_state(app);

    // TODO: Add a tracing layer.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    // More issues with sync postgres client being dependent on tokio.
    // Couldn't gracefully shutdonwn.

    //let signals = signal::ctrl_c();

    axum::serve(listener, router)
        //.with_graceful_shutdown(async move {
        //    if let Err(e) = signals.await {
        //        eprintln!("error during shutdown: {}", e);
        //    }
        //})
        .await?;

    Ok(())
}
