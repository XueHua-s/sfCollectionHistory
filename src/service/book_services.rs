use std::env;

use crate::model::book::Book;
use actix_web;
use reqwest;
use scraper::{Html, Selector};
use serde_json;
use uuid::Uuid;
pub struct BookServices;
impl BookServices {
    pub async fn add_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let new_book = Book {
            id: Some("你好世界".to_string()),
            b_id: book_id, // Default value, adjust as necessary
            book_name: String::from("Default Book Name"), // Default value
            tap_num: 3,
            book_type: String::from("Default Book Type"), // Default value
            tags: String::from("Default Tags"),           // Default value
            like_num: 0,                                  // Default value
            collect_num: 0,                               // Default value
            comment_num: 0,                               // Default value
            comment_long_num: 0,                          // Default value
            created_time: String::from("2023-01-01T00:00:00Z"), // Default value
        };
        Ok(new_book)
    }
    async fn find_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let base_head_url = env::var("SF_DATA_BASE_URL").expect("未获取到sf接口网址");
        let base_url = format!("{}{}", base_head_url.clone(), "/Novel/");
        let url = format!("{}{}", base_url, book_id); // Corrected the URL construction
        let mut title = String::from("");
        let mut book_type = String::new();
        let mut click_count = 0;
        let mut tags_string = String::from("");
        let mut like_num = 0;
        let mut collect_num = 0;
        let mut comment_num = 0;
        let mut comment_long_num = 0;
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
            let selector = Selector::parse(".title").unwrap();
            if let Some(element) = document.select(&selector).next() {
                if let Some(tag_value) = element.text().next() {
                    title.push_str(tag_value);
                }
            }
            // 查询书本类型和点击量
            let selector = Selector::parse(".count-detail .text-row .text").unwrap();
            for element in document.select(&selector) {
                if let Some(text) = element.text().next() {
                    if text.starts_with("类型：") {
                        book_type = text.replace("类型：", "").trim().to_string();
                    } else if text.starts_with("点击：") {
                        click_count = text
                            .replace("点击：", "")
                            .trim()
                            .parse::<i32>()
                            .unwrap_or(0);
                        // You can store click_count if needed, for now, we just extract it
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
            tags_string = tags.join(",");
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
        } else {
            return Err(actix_web::error::ErrorNotFound("Book not found")); // Return a not found error if the response is not successful
        }

        // 创建Book结构体，并返回Result结果
        let new_book = Book {
            id: Some(Uuid::new_v4().to_string()), // Use the fetched title instead of a default value
            b_id: book_id,
            book_name: title.clone(),
            book_type: book_type.clone(),
            tap_num: click_count,
            tags: tags_string.clone(),
            like_num,
            collect_num,
            comment_num,
            comment_long_num,
            created_time: chrono::Utc::now().format("%Y-%m-%d").to_string(), // Automatically generate the current time in YYYY-MM-DD format
        };
        Ok(new_book)
    }
}
