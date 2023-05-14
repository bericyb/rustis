use axum::{
    routing::{delete, get, patch, post},
    Json, Router,
};
use tokio::signal;

use state::Storage;

mod db;
mod rts;

static DB: Storage<db::Database> = Storage::new();

#[tokio::main]
async fn main() {
    println!("Starting Rustis!");

    println!("Creating hashmap...");
    DB.set(db::Database::new());

    println!("Building routes...");
    let app = Router::new()
        .route(
            "/ping",
            get(|| async {
                return Json("pong!");
            }),
        )
        .route("/:key", get(rts::read))
        .route("/:key", post(rts::create))
        .route("/:key", patch(rts::update))
        .route("/:key", delete(rts::delete));

    println!("Checking for existing database...");
    DB.get().read_from_file("database.json").await;


    println!("Server started");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");

    DB.get().write_to_file("database.json").await;
}
