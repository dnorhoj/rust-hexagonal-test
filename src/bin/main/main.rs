use dotenvy::dotenv;
use repositories::Postgres;
use routes::AppState;
use services::notes::{NoteService, Service};
use tokio::net::TcpListener;

mod domains;
mod repositories;
mod routes;
mod services;

#[tokio::main]
pub async fn main() {
    let _ = dotenv();

    let listener = TcpListener::bind(("127.0.0.1", 8000)).await.unwrap();

    let database_uri = std::env::var("DATABASE_URL").unwrap();
    let state = AppState::new(Service::new(Postgres::new(database_uri).await));

    let router = routes::get_router(state);

    axum::serve(listener, router).await.unwrap();
}
