use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};

use crate::{repositories::notes::NoteRepository, services::notes::NoteService};

#[derive(Clone)]
pub struct AppState {
    pub note_service: Arc<dyn NoteService>,
}

impl AppState {
    pub fn new(note_service: impl NoteService) -> Self {
        Self {
            note_service: Arc::new(note_service),
        }
    }
}

pub mod notes {
    use ::notes::CreateNote;

    use crate::domains::notes::{CreateNoteError, GetNotesError};

    use super::*;

    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/", get(get_notes))
            .route("/", post(create_note))
    }

    async fn get_notes(State(state): State<AppState>) -> Response {
        let AppState { note_service } = state;

        let notes = match note_service.get_notes().await {
            Ok(notes) => notes,
            Err(GetNotesError::Other) => {
                return (StatusCode::INTERNAL_SERVER_ERROR,).into_response();
            }
        };

        (StatusCode::OK, Json(notes)).into_response()
    }

    async fn create_note(
        State(state): State<AppState>,
        Json(req): Json<CreateNote>,
    ) -> Response {
        let AppState { note_service } = state;

        let note = match note_service.create_note(req).await {
            Ok(note) => {note},
            Err(CreateNoteError::Other) => {
                return (StatusCode::INTERNAL_SERVER_ERROR,).into_response();
            },
        };

        (StatusCode::OK, Json(note)).into_response()
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/notes", notes::routes())
        .with_state(state)
}
