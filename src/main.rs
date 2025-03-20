mod config;
mod route;

use config::Config;
use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    routing::{get, post},
    Router,
};

use shrink::{
    app::{App, AppState},
    storage::Redis,
    validator::{Alnum, Validator},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_client = Redis::default();

    let app = App::open("data/urls.db")?.with_cache(redis_client);
    let app = Arc::new(RwLock::new(app));

    let config = Config::from_env().unwrap_or_default();

    let app = AppState {
        app,
        base_url: config.server_url,
        validator: Arc::new(Validator::new(Alnum::default())),
    };

    let router = Router::new()
        .route("/", post(route::shrink).put(route::custom_code))
        .route("/{code}", get(route::redirect))
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
