use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
struct AppState {
    users: Arc<BTreeMap<u64, String>>,
}

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>, StatusCode> {
    let name = state.users.get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(UserResponse { id, name }))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/users/{id}", get(get_user))
        .with_state(state)
}
