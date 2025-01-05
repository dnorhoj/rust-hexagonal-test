use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoteTitleError {
    #[error("Note title is empty")]
    TitleEmptyError,
    #[error("Note title is too long")]
    TitleTooLongError,
}

#[derive(Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn new(str: String) -> Result<Self, NoteTitleError> {
        if str.is_empty() {
            return Err(NoteTitleError::TitleEmptyError);
        }

        if str.len() > 255 {
            return Err(NoteTitleError::TitleTooLongError);
        }

        Ok(Self(str))
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Note {
    pub id: String,
    pub title: Option<NoteTitle>,
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateNote {
    pub title: Option<String>,
    pub body: String,
}

impl CreateNote {
    pub fn new(title: Option<String>, body: String) -> Self {
        Self { title, body }
    }
}
