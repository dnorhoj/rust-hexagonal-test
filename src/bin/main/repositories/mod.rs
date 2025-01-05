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
    use crate::domains::notes::{CreateNoteError, DeleteNoteError, GetNotesError};
    use ::notes::{CreateNote, Note};

    use super::*;

    #[async_trait]
    pub trait NoteRepository: Send + Sync + 'static {
        async fn get_notes(&self) -> Result<Vec<Note>, GetNotesError>;
        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError>;
        async fn delete_note(&self, req: String) -> Result<(), DeleteNoteError>;
    }

    #[async_trait]
    impl NoteRepository for Postgres {
        async fn get_notes(&self) -> Result<Vec<Note>, GetNotesError> {
            let notes: Vec<Note> = match sqlx::query_as(r#"SELECT id, title, content FROM notes"#)
                .fetch_all(&self.pool)
                .await
            {
                Ok(notes) => notes,
                Err(_) => return Err(GetNotesError::Other),
            };

            Ok(notes)
        }

        async fn create_note(&self, req: CreateNote) -> Result<Note, CreateNoteError> {
            match sqlx::query(r"INSERT INTO note (title, body) VALUES ($1, $2)")
                .bind(req.title)
                .bind(req.body)
                .execute(&self.pool)
                .await
            {
                Ok(_) => todo!(),
                Err(_) => Err(CreateNoteError::Other),
            }
        }

        async fn delete_note(&self, _req: String) -> Result<(), DeleteNoteError> {
            todo!()
        }
    }
}
