use acme_web::helper::logger;
use acme_web::helper::server;

#[tokio::main]
async fn main() {
    logger::init();
    server::init().await;
}
