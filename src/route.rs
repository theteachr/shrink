use axum::{
    extract::{Path, State},
    response::Redirect,
    Json,
};
use shrink::{app::AppState, error, Shrinker, Storage};
use url::Url;

#[derive(serde::Serialize)]
pub struct ShrinkResponse {
    shrunk: Url,
}

#[derive(serde::Deserialize)]
pub struct ShrinkRequest {
    url: Url,
}

#[derive(serde::Deserialize)]
pub struct CustomShrinkRequest {
    alias: String,
    url: Url,
}

pub async fn shrink(
    State(state): State<AppState>,
    body: Json<ShrinkRequest>,
) -> Result<Json<ShrinkResponse>, error::Internal> {
    let ShrinkRequest { url } = body.0;
    // XXX: Maybe inefficient because of locking the entire database?
    let code = state.app.write().await.shrink(url)?;

    // #WET-02: Response generation
    state
        .shrink_response(&code)
        .ok_or(error::Internal("Failed to generate a code.".into()))
        .map(|url| Json(ShrinkResponse { shrunk: url }))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, error::Load> {
    // #WET-01: Validation
    let code = state
        .validator
        .validate(code)
        .ok_or(error::Load::BadAlias)?;
    let url = state.app.read().await.expand(&code)?;
    // Consider using 302 (Status Found) instead of 307 (Status Temporary Redirect).
    Ok(Redirect::temporary(url.as_str()))
}

pub async fn custom_code(
    State(state): State<AppState>,
    body: Json<CustomShrinkRequest>,
) -> Result<Json<ShrinkResponse>, error::Storage> {
    let CustomShrinkRequest { url, alias: code } = body.0;

    // #WET-01: Validation
    // XXX: Use a deserializer or middleware to DRY this up?
    let code = state
        .validator
        .validate(code)
        .ok_or(error::Storage::BadAlias)?;

    state.app.write().await.urls.store(url, &code)?;

    // #WET-02: Response generation
    state
        .shrink_response(&code)
        .ok_or(error::Storage::Internal(
            "Failed to generate a code.".into(),
        ))
        .map(|url| Json(ShrinkResponse { shrunk: url }))
}
