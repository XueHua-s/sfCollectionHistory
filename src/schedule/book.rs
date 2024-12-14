use crate::service::book_services;
use actix_web::Error;
use tokio_cron_scheduler::{Job, JobScheduler};

async fn push_sf_book_new_data(bid: i32) -> Result<(), Error> {
    let book = book_services::BookServices::find_sf_book(bid).await?;
    book_services::BookServices::insert_sf_book(book).await?;
    Ok(())
}

pub async fn schedule_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut sched = JobScheduler::new();

    let job = Job::new("06 15 * * *", |_uuid, _l| {
        async fn async_fn() {
            match book_services::BookServices::find_sf_all_bid().await {
                Ok(bids) => {
                    let mut tasks = Vec::new();
                    for id in bids {
                        tasks.push(push_sf_book_new_data(id));
                    }

                    // 等待所有任务完成
                    for task in tasks {
                        if let Err(err) = task.await {
                            eprintln!("Task failed: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch book IDs: {:?}", err);
                }
            }
        }
        // tokio提供异步运行时
        let _ = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async_fn());
    })?;

    let _ = sched.add(job);
    sched.start().await?;
    Ok(())
}
