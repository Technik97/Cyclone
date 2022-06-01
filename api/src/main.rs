use actix_web::{web, get, App, HttpServer, middleware, Responder, HttpResponse};
use listenfd::ListenFd;

use entity::db::app_state::AppState;
use entity::db::conn;
use entity::dotenv;
use migration::{Migrator, MigratorTrait};

mod controller;
use controller::user_controller as UserController;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("All Good")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let mut listenfd = ListenFd::from_env();

    let conn = conn::get_conn().await.to_owned();
    let server_url = conn::get_server_url();

    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let mut server = HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(UserController::create);
    cfg.service(health_check);
}