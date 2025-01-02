use std::sync::Arc;

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    *,
};
use http::StatusCode;

use crate::domains::notes::NoteRepository;

#[derive(Clone, Debug)]
pub struct AppState {
    pub note_repository: Arc<impl NoteRepository>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ost: "Hej".into(),
        }
    }
}

pub mod notes {
    use extract::State;

    use super::*;

    pub fn routes() -> Router<AppState> {
        Router::new().route("/", get(get_notes))
    }

    async fn get_notes(State(state): State<AppState>) -> Response {
        let AppState { ost } = state;

        (StatusCode::OK, ost).into_response()
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/notes", notes::routes())
        .with_state(state)
}
