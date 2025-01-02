use async_trait::async_trait;

pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new(connection_uri: String) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_uri.as_str())
            .await
            .unwrap();

        Self { pool }
    }
}

pub mod notes {
    use ::notes::{CreateNote, Note};
    use crate::domains::notes::{CreateNoteError, DeleteNoteError, NoteRepository};

    use super::*;

    #[async_trait]
    impl NoteRepository for Postgres {
        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError> {
            todo!()
        }

        async fn delete_note(&self, req: String) -> Result<Note, DeleteNoteError> {
            todo!()
        }
    }
}
