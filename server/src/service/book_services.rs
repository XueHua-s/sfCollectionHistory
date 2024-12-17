use crate::dto::book::PageQueryBookAnalysisRecordsReq;
use crate::{model::book::Book, mysql::client};
use actix_web::{self};
use reqwest;
use scraper::{Html, Selector};
use serde_json;
use sqlx;
use std::env;
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
    // 查询书本最新记录
    pub async fn get_book_new_once_detail(book_id: i32) -> Result<Book, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;

        // 查询books表中指定bid的最新记录(通过rid降序排列, rid是自增记录id)
        let sql = "
        SELECT id, b_id, book_name, cover_url, book_type, tap_num, tags, like_num, 
            collect_num, comment_num, comment_long_num, monthly_pass, 
            monthly_ticket_ranking, reward_ranking, 
            DATE_FORMAT(created_time, '%Y-%m-%d') as created_time,
            DATE_FORMAT(last_update_time, '%Y-%m-%d') as last_update_time,
            r_id
        FROM books
        WHERE b_id = ?
        ORDER BY r_id DESC
        LIMIT 1;
    ";

        let row: (
            Option<String>,
            i32,
            String,
            String,
            String,
            i32,
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            String,
            String,
        ) = sqlx::query_as(sql)
            .bind(book_id)
            .fetch_one(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;

        // 将查询结果转换为Book结构体
        let new_book = Book {
            id: row.0,
            b_id: row.1,
            book_name: row.2,
            cover_url: row.3,
            book_type: row.4,
            tap_num: row.5,
            tags: row.6,
            like_num: row.7,
            collect_num: row.8,
            comment_num: row.9,
            comment_long_num: row.10,
            monthly_pass: row.11,
            monthly_ticket_ranking: row.12,
            reward_ranking: row.13,
            created_time: row.14,
            last_update_time: row.15,
        };

        Ok(new_book)
    }
    // 分页查询书本分析记录
    pub async fn page_query_book_analysis_records(
        query: PageQueryBookAnalysisRecordsReq,
    ) -> Result<Vec<Book>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;
        let default_sql = "
                SELECT id, b_id, book_name, book_type, tags, like_num, collect_num, comment_num, comment_long_num, tap_num, monthly_pass, monthly_ticket_ranking, reward_ranking, cover_url, DATE_FORMAT(last_update_time, '%Y-%m-%d') AS last_update_time, DATE_FORMAT(created_time, '%Y-%m-%d') AS created_time
                FROM books
                WHERE created_time BETWEEN ? AND ? AND b_id = ?
                ORDER BY created_time DESC;
            ".to_string();
        let sql = match query.group_type {
            Some (1) => default_sql,
            Some(num) => {
                let mut goupsql = String::new();
                match num {
                    2 => goupsql.push_str("%Y-%m"),
                    3 => goupsql.push_str("%Y"),
                    _ => goupsql.push_str("SELECT ..."), 
                };
                format!("WITH cte AS (
                    SELECT 
                        DATE_FORMAT(created_time, '{}') AS month,
                        id,
                        b_id,
                        book_name,
                        book_type,
                        tags,
                        CAST(like_num AS SIGNED) AS like_num,
                        CAST(collect_num AS SIGNED) AS collect_num,
                        CAST(comment_num AS SIGNED) AS comment_num,
                        CAST(comment_long_num AS SIGNED) AS comment_long_num,
                        CAST(tap_num AS SIGNED) AS tap_num,
                        CAST(monthly_pass AS SIGNED) AS monthly_pass,
                        CAST(monthly_ticket_ranking AS SIGNED) AS monthly_ticket_ranking,
                        CAST(reward_ranking AS SIGNED) AS reward_ranking,
                        cover_url,
                        last_update_time,
                         ROW_NUMBER() OVER (PARTITION BY DATE_FORMAT(created_time, '{}') ORDER BY r_id DESC) AS rn,
                        r_id
                    FROM books
                    WHERE created_time BETWEEN ? AND ? AND b_id = ?
                )
                SELECT
                    MAX(CASE WHEN rn = 1 THEN id END) AS id,
                    MAX(CASE WHEN rn = 1 THEN b_id END) AS b_id,
                    MAX(CASE WHEN rn = 1 THEN book_name END) AS book_name,
                    MAX(CASE WHEN rn = 1 THEN book_type END) AS book_type,
                    MAX(CASE WHEN rn = 1 THEN tags END) AS tags,
                    CAST(SUM(like_num) AS SIGNED) AS like_num,
                    CAST(SUM(collect_num) AS SIGNED) AS collect_num,
                    CAST(SUM(comment_num) AS SIGNED) AS comment_num,
                    CAST(SUM(comment_long_num) AS SIGNED) AS comment_long_num,
                    CAST(SUM(tap_num) AS SIGNED) AS tap_num,
                    CAST(SUM(monthly_pass) AS SIGNED) AS monthly_pass,
                    CAST(SUM(monthly_ticket_ranking) AS SIGNED) AS monthly_ticket_ranking,
                    CAST(SUM(reward_ranking) AS SIGNED) AS reward_ranking,
                    MAX(CASE WHEN rn = 1 THEN cover_url END) AS cover_url,
                    MAX(CASE WHEN rn = 1 THEN DATE_FORMAT(last_update_time, '%Y-%m-%d') END) AS last_update_time,
                    month AS created_time
                FROM cte
                GROUP BY created_time", goupsql, goupsql)
            },
            _ => default_sql,
        };
    
        let rows: Vec<Book> = sqlx::query_as::<
            _,
            (
                Option<String>, // id
                i32,            // b_id
                String,         // book_name
                String,         // book_type
                String,         // tags
                i32,            // like_num
                i32,            // collect_num
                i32,            // comment_num
                i32,            // comment_long_num
                i32,         // created_time
                i32,            // tap_num
                i32,            // monthly_pass
                i32,            // monthly_ticket_ranking
                String,            // reward_ranking
                String,
                String,
            ),
        >(&sql)
        .bind(&query.start_date)
        .bind(&query.end_date)
        .bind(query.b_id)
        // .bind(query.size)
        // .bind((query.current - 1) * query.size)
        .fetch_all(&*client)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
        })?
        .into_iter()
        .map(|row| Book {
            id: row.0,
            b_id: row.1,
            book_name: row.2,
            book_type: row.3,
            tags: row.4,
            like_num: row.5,
            collect_num: row.6,
            comment_num: row.7,
            comment_long_num: row.8,
            tap_num: row.9,
            monthly_pass: row.10,
            monthly_ticket_ranking: row.11,
            reward_ranking: row.12,
            cover_url: row.13,
            last_update_time: row.14,
            created_time: row.15,
        })
        .collect();
    
        Ok(rows)
    }
    

    // 暴露给控制器, 用于恢复维护
    pub async fn to_book_maintenance(book_id: i32) -> Result<Book, actix_web::Error> {
        if Self::has_this_book(book_id).await? {
            let book = Self::find_sf_book(book_id).await?;
            let new_record = Self::get_book_new_once_detail(book_id).await?;

            let lash_update_time =
                chrono::NaiveDate::parse_from_str(&book.last_update_time.clone(), "%Y/%m/%d")
                    .unwrap();
            let new_record_time =
                chrono::NaiveDate::parse_from_str(&new_record.last_update_time.clone(), "%Y/%m/%d")
                    .unwrap();
            let current_date = chrono::Local::now().date_naive();
            if current_date
                .signed_duration_since(new_record_time)
                .num_days()
                < 30
            {
                // 本书处于维护中
                return Err(actix_web::error::ErrorBadRequest("book_state_maintenance"));
            }
            if current_date
                .signed_duration_since(lash_update_time)
                .num_days()
                > 30
            {
                // 本书当前状态超过最大维护时间
                return Err(actix_web::error::ErrorBadRequest("maintenance_max"));
            }
            // 添加记录重新进入维护状态
            let res = Self::insert_sf_book(book).await?;
            return Ok(res);
        }
        Err(actix_web::error::ErrorBadRequest("not_has_book"))
    }
    // 将数据插入表中
    pub async fn insert_sf_book(new_book: Book) -> Result<Book, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;

        let sql = "INSERT INTO books (id, b_id, book_name, cover_url, book_type, tags, like_num, collect_num, comment_num, comment_long_num, created_time, tap_num, monthly_pass, monthly_ticket_ranking, reward_ranking, last_update_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
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
            .bind(&new_book.created_time)
            .bind(new_book.tap_num)
            .bind(new_book.monthly_pass)
            .bind(new_book.monthly_ticket_ranking)
            .bind(new_book.reward_ranking)
            .bind(&new_book.last_update_time)
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
    // 查询在维护所有的书本bid
    pub async fn find_sf_all_bid() -> Result<Vec<i32>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;
        // 分组查询最新的创建时间那条, 取最近更新时间 >= 30天的。
        let sql = "
           SELECT b_id, MAX(created_time) as max_created_time
            FROM books
            WHERE last_update_time >= DATE_SUB(CURDATE(), INTERVAL 30 DAY)
            GROUP BY b_id;
        ";

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
        let mut last_update_time = String::new();
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
                    } else if text.starts_with("更新：") {
                        last_update_time = text.replace("更新：", "").trim().to_string();
                        // 最后更新时间
                        if let Some(date_part) = last_update_time.split_whitespace().next() {
                            last_update_time = date_part.to_string();
                        }
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
        let new_book = Book::from(Book {
            id: None, // Use the fetched title instead of a default value
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
            last_update_time,
            created_time: String::new(), // Automatically generate the current time in YYYY-MM-DD format
        });
        Ok(new_book)
    }
}
