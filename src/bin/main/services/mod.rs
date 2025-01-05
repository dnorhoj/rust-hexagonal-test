pub mod notes {
    use async_trait::async_trait;
    use notes::{CreateNote, Note};

    use crate::{
        domains::notes::{CreateNoteError, DeleteNoteError, GetNotesError},
        repositories::notes::NoteRepository,
    };

    #[async_trait]
    pub trait NoteService: Send + Sync + 'static {
        async fn get_notes(&self) -> Result<Vec<Note>, GetNotesError>;
        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError>;
        async fn delete_note(&self, req: String) -> Result<(), DeleteNoteError>;
    }

    pub struct Service<R: NoteRepository> {
        repo: R,
    }

    impl<R: NoteRepository> Service<R> {
        pub fn new(repo: R) -> Self {
            Self { repo }
        }
    }

    #[async_trait]
    impl<R: NoteRepository> NoteService for Service<R> {
        async fn get_notes(&self) -> Result<Vec<Note>, GetNotesError> {
            self.repo.get_notes().await
        }

        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError> {
            self.repo.create_note(req).await
        }

        async fn delete_note(&self, req: String) -> Result<(), DeleteNoteError> {
            self.repo.delete_note(req).await
        }
    }
}
