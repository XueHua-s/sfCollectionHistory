use actix_web::http::header::{self, HeaderValue};
use actix_web::middleware;
use actix_web::{App, HttpServer};
use blog::modules;
use blog::schedule::book;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // 创建定时任务线程
    tokio::spawn(async {
        let _ = book::schedule_task().await;
    });
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add((
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            )))
            .wrap(middleware::Logger::default())
            .configure(modules::configure)
    })
    // 配置线城池数量
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
