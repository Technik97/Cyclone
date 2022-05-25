use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use listenfd::ListenFd;

use entity::db::app_state::AppState;
use entity::db::conn;
use entity::dotenv;
use migration::{Migrator, MigratorTrait};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}
