use routes::AppState;
use tokio::net::TcpListener;

mod routes;
mod repositories;
mod domains;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 8000)).await.unwrap();

    let state = AppState::new();

    let router = routes::get_router(state);

    axum::serve(listener, router).await.unwrap();
}
