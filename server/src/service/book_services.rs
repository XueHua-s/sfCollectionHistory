use std::env;

use crate::{model::book::Book, mysql::client};
use actix_web::{self};
use reqwest;
use scraper::{Html, Selector};
use serde_json;
use sqlx;
use uuid::Uuid;
pub struct BookServices;
impl BookServices {
    // 暴露给控制器, 用于收录书本的api
    pub async fn add_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        if Self::has_this_book(book_id).await? {
            return Err(actix_web::error::ErrorBadRequest("has_book"));
        }
        let new_book = Self::find_sf_book(book_id).await?;
        let _ = Self::insert_sf_book(new_book.clone()).await;
        Ok(new_book)
    }
    // 将数据插入表中
    pub async fn insert_sf_book(new_book: Book) -> Result<Book, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;

        let sql = "INSERT INTO books (id, b_id, book_name, cover_url, book_type, tags, like_num, collect_num, comment_num, comment_long_num, created_time, tap_num, monthly_pass, monthly_ticket_ranking, reward_ranking) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), ?, ?, ?, ?)";
        sqlx::query(sql)
            .bind(new_book.id.clone())
            .bind(new_book.b_id) // 使用b_id而不是id
            .bind(&new_book.book_name)
            .bind(&new_book.cover_url)
            .bind(&new_book.book_type)
            .bind(&new_book.tags)
            .bind(new_book.like_num)
            .bind(new_book.collect_num)
            .bind(new_book.comment_num)
            .bind(new_book.comment_long_num)
            .bind(new_book.tap_num)
            .bind(new_book.monthly_pass)
            .bind(new_book.monthly_ticket_ranking)
            .bind(new_book.reward_ranking)
            .execute(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database insert error: {}", e))
            })?;
        Ok(new_book)
    }
    // 查询这本书存不存在表记录
    async fn has_this_book(book_id: i32) -> Result<bool, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;

        let sql = "SELECT EXISTS(SELECT 1 FROM books WHERE b_id = ?)";

        let record_exists: (bool,) = sqlx::query_as(sql)
            .bind(book_id)
            .fetch_one(&*client)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => actix_web::error::ErrorNotFound("Book not found"),
                _ => actix_web::error::ErrorInternalServerError(format!(
                    "Database query error: {}",
                    e.to_string()
                )),
            })?;

        Ok(record_exists.0)
    }
    // 查询所有的书本bid
    pub async fn find_sf_all_bid() -> Result<Vec<i32>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;

        let sql = "SELECT DISTINCT b_id FROM books"; // Use DISTINCT to get unique b_id values
        let bids: Vec<i32> = sqlx::query_scalar(sql)
            .fetch_all(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;

        Ok(bids)
    }
    // 爬虫, 爬取这本书的数据
    pub async fn find_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let base_head_url = env::var("SF_DATA_BASE_URL").expect("未获取到sf接口网址");
        let base_url = format!("{}{}", base_head_url.clone(), "/Novel/");
        let url = format!("{}{}", base_url, book_id); // Corrected the URL construction
        let mut title = String::from("");
        let mut cover_url = String::new();
        let mut book_type = String::new();
        let mut click_count = 0;
        let mut tags_string = String::new();
        let mut like_num = 0;
        let mut collect_num = 0;
        let mut comment_num = 0;
        let mut comment_long_num = 0;
        let mut monthly_pass = 0;
        let mut monthly_ticket_ranking = 0;
        let mut reward_ranking = 0;
        // 发送 GET 请求
        let response = reqwest::get(&url)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; // Map reqwest error to actix_web error

        // 确保请求成功
        if response.status().is_success() {
            // 获取响应的文本内容
            let body = response
                .text()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            // 解析 HTML
            let document = Html::parse_document(&body);
            // 查询书本标题
            let selector = Selector::parse("h1.title .text").unwrap();
            if let Some(element) = document.select(&selector).next() {
                if let Some(tag_value) = element.text().next() {
                    title.push_str(tag_value.trim());
                }
            }
            // 查询书本类型和点击量
            let selector = Selector::parse(".count-detail .text-row .text").unwrap();
            for element in document.select(&selector) {
                if let Some(text) = element.text().next() {
                    if text.starts_with("类型：") {
                        book_type = text.replace("类型：", "").trim().to_string();
                    } else if text.starts_with("点击：") {
                        let click_text = text.replace("点击：", "").trim().to_string();
                        click_count = match click_text.strip_suffix("万") {
                            Some(value) => {
                                (value.trim().parse::<f32>().unwrap_or(0.0) * 10_000.0) as i32
                            }
                            None => match click_text.strip_suffix("千") {
                                Some(value) => {
                                    (value.trim().parse::<f32>().unwrap_or(0.0) * 1_000.0) as i32
                                }
                                None => click_text.parse::<i32>().unwrap_or(0),
                            },
                        };
                    }
                }
            }
            // 查询书本标签
            let mut tags = Vec::new();
            let tag_selector = Selector::parse(".tag-list .tag .text").unwrap();
            for element in document.select(&tag_selector) {
                if let Some(tag_value) = element.text().next() {
                    tags.push(tag_value.to_string());
                }
            }
            let tags = tags.join(",");
            tags_string.push_str(&tags);
            // 查询点赞数量
            let like_selector = Selector::parse("#BasicOperation .btn.yellow").unwrap();
            for element in document.select(&like_selector) {
                if let Some(text) = element.text().next() {
                    if text.starts_with("赞 ") {
                        let like_count_str = text.replace("赞 ", "");
                        like_num = like_count_str.trim().parse::<i32>().unwrap_or(0);
                    } else if text.starts_with("收藏 ") {
                        let collect_num_str = text.replace("收藏 ", "");
                        collect_num = collect_num_str.trim().parse::<i32>().unwrap_or(0);
                    }
                }
            }
            // 查询评论数
            let comments_url = format!(
                "{}/ajax/ashx/Common.ashx?op=getcomment&nid={}&_={}",
                base_head_url.clone(),
                book_id,
                chrono::Utc::now().timestamp_millis()
            );
            let comments_response = reqwest::get(&comments_url)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            if comments_response.status().is_success() {
                let comments_data: serde_json::Value = comments_response
                    .json()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                if comments_data["status"] == 200 {
                    comment_num = comments_data["ShortCommentNum"].as_i64().unwrap_or(0) as i32;
                    comment_long_num = comments_data["LongCommentNum"].as_i64().unwrap_or(0) as i32;
                }
            } else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Failed to fetch comments",
                ));
            }
            // 查询书本封面
            let cover_selector = Selector::parse(".books-box .left-part .figure .pic img").unwrap();
            if let Some(cover_element) = document.select(&cover_selector).next() {
                if let Some(url) = cover_element.value().attr("src") {
                    cover_url.push_str(url);
                }
            } else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Failed to fetch cover image",
                ));
            }
            // 查询今日月票数据
            let ticket_info_url = format!(
                "{}{}",
                base_head_url, "/ajax/ashx/Common.ashx?op=ticketinfo"
            );

            let ticket_info_response = reqwest::Client::new()
                .post(&ticket_info_url)
                .form(&[("nid", book_id.to_string())])
                .send()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            if ticket_info_response.status().is_success() {
                let ticket_data: serde_json::Value = ticket_info_response
                    .json()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                if ticket_data["status"] == 200 {
                    monthly_pass = ticket_data["tickets"]["TicketNum"].as_i64().unwrap_or(0) as i32;
                    monthly_ticket_ranking =
                        ticket_data["tickets"]["Rank"].as_i64().unwrap_or(0) as i32;
                }
            } else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Failed to fetch ticket info",
                ));
            }
            // 查询今日打赏数据
            let bonus_info_url =
                format!("{}{}", base_head_url, "/ajax/ashx/Common.ashx?op=bonusinfo");

            let bonus_info_response = reqwest::Client::new()
                .post(&bonus_info_url)
                .form(&[("nid", book_id.to_string())])
                .send()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            if bonus_info_response.status().is_success() {
                let bonus_data: serde_json::Value = bonus_info_response
                    .json()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                if bonus_data["status"] == 200 {
                    let rank = bonus_data["bonus"]["Rank"].as_i64().unwrap_or(0) as i32;
                    reward_ranking = rank;
                }
            } else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Failed to fetch bonus info",
                ));
            }
        } else {
            return Err(actix_web::error::ErrorNotFound("Book not found")); // Return a not found error if the response is not successful
        }

        // 创建Book结构体，并返回Result结果
        let new_book = Book {
            id: Some(Uuid::new_v4().to_string()), // Use the fetched title instead of a default value
            b_id: book_id,
            book_name: title.clone(),
            cover_url,
            book_type: book_type.clone(),
            tap_num: click_count,
            tags: tags_string.clone(),
            like_num,
            collect_num,
            comment_num,
            comment_long_num,
            monthly_pass,
            monthly_ticket_ranking,
            reward_ranking,
            created_time: chrono::Utc::now().format("%Y-%m-%d").to_string(), // Automatically generate the current time in YYYY-MM-DD format
        };
        Ok(new_book)
    }
}