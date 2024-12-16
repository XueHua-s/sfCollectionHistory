use crate::service::book_services;
use actix_web::Error;
use cron::Schedule;
use std::{str::FromStr, sync::Arc};
use tokio::{
    sync::Semaphore,
    time::{sleep_until, Duration, Instant},
};
async fn push_sf_book_new_data(bid: i32) -> Result<(), Error> {
    let mut book = book_services::BookServices::find_sf_book(bid).await;
    while book.is_err() {
        book = book_services::BookServices::find_sf_book(bid).await;
    }
    let book = book?;
    loop {
        if book_services::BookServices::insert_sf_book(book.clone())
            .await
            .is_ok()
        {
            break;
        }
    }
    Ok(())
}
async fn async_fn() -> Result<(), actix_web::Error> {
    let bids = book_services::BookServices::find_sf_all_bid().await?;
    let mut tasks = Vec::new();
    // 使用信号量来控制并发数，优化单线程并发效率
    let semaphore = Arc::new(Semaphore::new(50));
    for id in bids {
        // 克隆Arc以共享Semaphore
        let semaphore_clone = semaphore.clone();
        tasks.push(async move {
            // 获取许可，如果Semaphore中没有可用的许可，这里会等待直到有许可可用
            let permit = semaphore_clone.acquire_owned().await.unwrap();
            // 执行耗费资源的异步任务
            let res = push_sf_book_new_data(id).await;
            // 显式释放许可，让其他等待的任务可以获取许可
            drop(permit);
            res
        })
    }
    Ok(())
}

use chrono::Local; // Import Local for local time

pub async fn schedule_task() {
    // 定义一个 cron 表达式，例如每天的 14:30
    let cron_expr = "00 32 24 * * *"; // 秒 分 时 日 月 星期

    // 解析 cron 表达式
    let schedule = Schedule::from_str(cron_expr).unwrap();

    loop {
        // 获取下一个运行时间
        let now = Local::now(); // Use local time instead of Utc
        let upcoming = schedule.upcoming(Local).take(1).next().unwrap(); // Use Local for upcoming time
                                                                         // 计算需要等待的时间
        let duration = upcoming.signed_duration_since(now).to_std().unwrap();

        // 等待直到下一个运行时间
        sleep_until(Instant::now() + Duration::from_secs(duration.as_secs())).await;

        // 执行任务
        println!("book爬虫维护任务开始执行: {}", Local::now()); // Use local time for task execution time

        // 任务函数
        let _ = async_fn().await;
        println!("book爬虫维护定时任务执行完成: {}", Local::now())
    }
}
