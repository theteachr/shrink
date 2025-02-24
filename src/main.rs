mod config;

use config::Config;
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
    error::{self, Internal, Load},
    storage::{Cached, Redis, Sqlite},
    Slug, Storage, Validator,
};

use shrink::{generators::RB62, Shrinker};
use url::Url;

async fn shrink(
    State(state): State<AppState>,
    body: Json<ShrinkRequest>,
) -> Result<Json<ShrinkResponse>, Internal> {
    let ShrinkRequest { url } = body.0;
    // XXX: Maybe inefficient because of locking the entire database?
    let slug = state.app.write().await.shrink(url)?;
    Ok(Json(state.shrink_response(&slug)))
}

async fn redirect(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Redirect, error::Load> {
    // #wet
    let slug = state.validator.validate(&slug).ok_or(Load::BadAlias)?;
    let url = state.app.read().await.expand(&slug)?;
    // Consider using 302 (Status Found) instead of 307 (Status Temporary Redirect).
    Ok(Redirect::temporary(url.as_str()))
}

async fn custom_slug(
    State(state): State<AppState>,
    body: Json<CustomShrinkRequest>,
) -> Result<Json<ShrinkResponse>, error::Storage> {
    let CustomShrinkRequest { url, alias: slug } = body.0;

    // #wet
    // XXX: Use a deserializer or middleware to DRY this up?
    let slug = state
        .validator
        .validate(&slug)
        .ok_or(error::Storage::BadAlias)?;

    state.app.write().await.urls.store(url, &slug)?;
    Ok(Json(state.shrink_response(&slug)))
}

#[derive(Clone)]
struct AppState {
    app: Arc<RwLock<App<RB62, Cached<Redis, Sqlite>>>>,
    base_url: Url,
    validator: Arc<Validator>,
}

impl AppState {
    fn shrink_response(&self, slug: &Slug) -> ShrinkResponse {
        ShrinkResponse {
            shrunk: self.base_url.join(slug.as_str()).unwrap(),
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
    alias: String,
    url: Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_client = Redis::default();

    let app = App::open("data/urls.db")?.with_cache(redis_client);
    let app = Arc::new(RwLock::new(app));

    let config = Config::from_env().unwrap_or_default();

    let app = AppState {
        app,
        validator: Arc::new(Validator::default()),
        base_url: config.server_url,
    };

    let router = Router::new()
        .route("/", post(shrink).put(custom_slug))
        .route("/{slug}", get(redirect))
        .with_state(app);

    // TODO: Add a tracing layer.

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{port}", port = config.port)).await?;

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
