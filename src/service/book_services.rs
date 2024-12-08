use actix_web;
use crate::model::book::Book;
use reqwest;
use scraper::{Html, Selector};
pub  struct BookServices;
impl BookServices {
    pub async fn add_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let new_book = Book {
            id: Some("你好世界".to_string()),
            b_id: book_id, // Default value, adjust as necessary
            book_name: String::from("Default Book Name"), // Default value
            tap_num: 3,
            book_type: String::from("Default Book Type"), // Default value
            tags: String::from("Default Tags"), // Default value
            like_num: 0, // Default value
            collect_num: 0, // Default value
            comment_num: 0, // Default value
            comment_long_num: 0, // Default value
            created_time: String::from("2023-01-01T00:00:00Z"), // Default value
        };
        Ok(new_book)
    }
    async fn find_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let base_url = "https://book.sfacg.com/Novel/";
        let url = format!("{}{}", base_url, book_id); // Corrected the URL construction
        let mut title = String::from("");
        let mut book_type  = String::new();
        let mut click_count = 0;
        // 发送 GET 请求
        let response = reqwest::get(&url).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?; // Map reqwest error to actix_web error
        
        // 确保请求成功
        if response.status().is_success() {
            // 获取响应的文本内容
            let body = response.text().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
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
                        click_count = text.replace("点击：", "").trim().parse::<i32>().unwrap_or(0);
                        // You can store click_count if needed, for now, we just extract it
                    }
                }
            }
        } else {
            return Err(actix_web::error::ErrorNotFound("Book not found")); // Return a not found error if the response is not successful
        }
        
        // Create a new Book instance with default values
        let new_book = Book {
            id: Some(title.clone()), // Use the fetched title instead of a default value
            b_id: book_id,
            book_name: title.clone(),
            book_type: book_type.clone(),
            tap_num: 3,
            tags: String::from("Default Tags"),
            like_num: 0,
            collect_num: 0,
            comment_num: 0,
            comment_long_num: 0,
            created_time: String::from("2023-01-01T00:00:00Z"),
        };
        Ok(new_book)
    }
}
