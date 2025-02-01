use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Router,
};

use shrink::{generators::RB62, Shrinker};
use shrink::{shrinkers::Basic, storage::Postgres};

#[derive(Clone)]
struct AppState {
    main: Arc<RwLock<Basic<RB62, Postgres>>>,
    scheme: &'static str,
    host: &'static str,
}

async fn custom_code(State(app): State<AppState>, body: String) -> Result<String, &'static str> {
    let (code, uri) = body.split_once(' ').ok_or("invalid body")?;
    let uri = uri.parse().map_err(|_| "invalid uri")?;

    let _ = app.main.write().await.store_custom(uri, code.to_string())?;

    let shortened_uri = format!(
        "{scheme}://{host}/{code}\n",
        scheme = app.scheme,
        host = app.host,
        code = code,
    );

    Ok(shortened_uri)
}

async fn shrink(State(app): State<AppState>, body: String) -> Result<String, &'static str> {
    let uri = body.parse().map_err(|_| "invalid uri")?;
    // XXX: Maybe inefficient because locking the entire database.
    let code = app.main.write().await.shrink(uri)?;

    let shortened_uri = format!(
        "{scheme}://{host}/{code}\n",
        scheme = app.scheme,
        host = app.host,
        code = code,
    );

    Ok(shortened_uri)
}

async fn redirect(
    State(app): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, StatusCode> {
    let uri = app
        .main
        .read()
        .await
        .expand(code)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Consider using 302 (Status Found) instead of 307 (Status Temporary Redirect).
    Ok(Redirect::temporary(&uri.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Basic::default();
    let app = Arc::new(RwLock::new(app));

    let app = AppState {
        main: app,
        scheme: "http",
        host: "localhost:3000",
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
