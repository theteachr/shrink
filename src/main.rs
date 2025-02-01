use std::sync::{Arc, RwLock};

use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
    Router,
};

use shrink::shrinkers::Basic;
use shrink::Shrinker;

// which calls one of these handlers
async fn root() -> &'static str {
    "H"
}

type AppState = Arc<RwLock<Basic>>;

async fn shrink(State(app): State<AppState>, body: String) -> Result<String, &'static str> {
    let uri = Uri::try_from(body).map_err(|_| "invalid uri")?;
    app.write().unwrap().shrink(uri)
}

async fn redirect(
    State(app): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, StatusCode> {
    let uri = app
        .read()
        .unwrap()
        .expand(code)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Redirect::temporary(&uri.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Basic::from_file("uris.txt")?;
    let app = Arc::new(RwLock::new(app));

    let router = Router::new()
        .route("/", get(root).post(shrink))
        .route("/{code}", get(redirect))
        .with_state(app);

    // TODO: Add a tracing layer.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
