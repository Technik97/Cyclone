use entity::tokio;
use api::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}