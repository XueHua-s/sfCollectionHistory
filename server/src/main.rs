// use actix_web::http::header;
use actix_web::middleware;
use actix_web::{App, HttpServer};
use blog::modules;
use tokio::task;
use tokio::runtime::Runtime;
use blog::schedule::book;
use env_logger;
use actix_cors::Cors;
use std::fs::File;
use log::{LevelFilter};
use env_logger::Builder;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    {
        // 在 debug 模式下，初始化日志到控制台
        env_logger::init();
    }

    #[cfg(not(debug_assertions))]
    {
        // 在 release 模式下，初始化日志到文件
        let file = File::create("app.log").unwrap();
        Builder::new()
            .filter(None, LevelFilter::Info)
            .write_style(env_logger::WriteStyle::Always)
            .target(env_logger::Target::Pipe(Box::new(file)))
            .init();
    }
    // 创建定时任务线程
    // 在阻塞任务中启动新的 Tokio 运行时
    task::spawn_blocking(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut feature = Vec::new();
            // 添加定时任务
            feature.push(book::schedule_task());
            // 等待定时任务,不要让定时任务关闭
            for task in feature {
                task.await;
            }
        });
    });
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::new(
                "ip: %a \"%r\" %s %b \"%{User-Agent}i\" %Dms"
            ))
            .configure(modules::configure)
    })
    // 配置线城池数量
    // .workers(4)
    .bind(("192.168.50.146", 8081))?
    .run()
    .await
}
