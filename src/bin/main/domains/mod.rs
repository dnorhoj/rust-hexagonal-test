use async_trait::async_trait;

pub mod notes {
    use ::notes::{CreateNote, Note};

    use super::*;

    pub enum CreateNoteError {
        Other,
    }

    pub enum DeleteNoteError {
        Other,
    }

    #[async_trait]
    pub trait NoteRepository: Send + Sync + 'static {
        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError>;
        async fn delete_note(&self, req: String) -> Result<Note, DeleteNoteError>;
    }
}
