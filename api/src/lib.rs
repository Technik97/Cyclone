use actix_web::{web, get, App, HttpServer, middleware, HttpResponse};
use actix_web::dev::Server;
use listenfd::ListenFd;
use actix_cors::Cors;

use entity::db::app_state::AppState;
use entity::db::conn;
use entity::dotenv;
use migration::{Migrator, MigratorTrait};

mod controller;
use controller::user_controller as UserController;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish() //.json("All Good")
}


pub fn run() -> Result<Server, std::io::Error> {
    // std::env::set_var("RUST_LOG", "debug");
    // tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    // let mut listenfd = ListenFd::from_env();

    // let conn = conn::get_conn().await.to_owned();
    // let server_url = conn::get_server_url();

    // Migrator::up(&conn, None).await.unwrap();

    // let state = AppState { conn };

    let mut server = HttpServer::new(move || {
        // let cors = Cors::permissive();

        App::new()
            // .data(state.clone())
            // .wrap(middleware::Logger::default())
            // .wrap(cors)
            .configure(init)
    })
    .bind("127.0.0.1:8000")?
    .run();

    // server = match listenfd.take_tcp_listener(0)? {
    //     Some(listener) => server.listen(listener)?,
    //     None => server.bind(&server_url)?,
    // };

    // println!("Starting server at {}", server_url);
    // server.run();

    Ok(server)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // cfg.service(UserController::create);
    cfg.service(health_check);
}
