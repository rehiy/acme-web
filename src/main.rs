use acme_web::helper::httpd;
use acme_web::helper::logger;

#[tokio::main]
async fn main() {
    logger::init();
    httpd::listen().await;
}
