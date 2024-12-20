use crate::dto;
use crate::dto::book::PageQueryBookAnalysisRecordsReq;
use crate::model::book::BookRank;
use crate::model::response::{ResponsPagerList, ResponsPagerListFrom};
use crate::{model::book::Book, mysql::client};
use actix_web::{self};
use regex::Regex;
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
        SELECT id, b_id, book_name, cover_url, finish, word_count, book_type, tap_num, tags, like_num, 
            collect_num, comment_num, comment_long_num, monthly_pass, 
            monthly_ticket_ranking, reward_ranking, 
            DATE_FORMAT(created_time, '%Y-%m-%d') as created_time,
            DATE_FORMAT(last_update_time, '%Y-%m-%d') as last_update_time,
            r_id,
            label_type,
        FROM books
        WHERE b_id = ?
        ORDER BY r_id DESC
        LIMIT 1;
    ";

        let row: Book = sqlx::query_as(sql)
            .bind(book_id)
            .fetch_one(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;
        Ok(row)
    }
    // 时间查询书本分析记录
    pub async fn page_query_book_analysis_records(
        query: PageQueryBookAnalysisRecordsReq,
    ) -> Result<Vec<Book>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;
        let default_sql = "
                SELECT id, b_id, book_name, finish, word_count, book_type, tags, like_num, collect_num, comment_num, comment_long_num, tap_num, monthly_pass, monthly_ticket_ranking, reward_ranking, cover_url, DATE_FORMAT(last_update_time, '%Y-%m-%d') AS last_update_time, DATE_FORMAT(created_time, '%Y-%m-%d') AS created_time, label_type
                FROM books
                WHERE created_time BETWEEN ? AND ? AND b_id = ?
                ORDER BY created_time DESC;
            ".to_string();
        let sql = match query.group_type {
            Some(1) => default_sql,
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
                        finish,
                        word_count,
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
                        r_id,
                        label_type
                    FROM books
                    WHERE created_time BETWEEN ? AND ? AND b_id = ?
                )
                SELECT
                    MAX(CASE WHEN rn = 1 THEN id END) AS id,
                    MAX(CASE WHEN rn = 1 THEN b_id END) AS b_id,
                    MAX(CASE WHEN rn = 1 THEN book_name END) AS book_name,
                    MAX(CASE WHEN rn = 1 THEN book_type END) AS book_type,
                    MAX(CASE WHEN rn = 1 THEN tags END) AS tags,
                    MAX(CASE WHEN rn = 1 THEN word_count END) AS word_count,
                    MAX(CASE WHEN rn = 1 THEN finish END) AS finish,
                    MAX(like_num) AS like_num,
                    MAX(collect_num) AS collect_num,
                    MAX(comment_num) AS comment_num,
                    MAX(comment_long_num) AS comment_long_num,
                    MAX(tap_num) AS tap_num,
                    MAX(monthly_pass) AS monthly_pass,
                    MAX(monthly_ticket_ranking) AS monthly_ticket_ranking,
                    MAX(reward_ranking) AS reward_ranking,
                    MAX(CASE WHEN rn = 1 THEN cover_url END) AS cover_url,
                    MAX(CASE WHEN rn = 1 THEN DATE_FORMAT(last_update_time, '%Y-%m-%d') END) AS last_update_time,
                    month AS created_time,
                    MAX(CASE WHEN rn = 1 THEN label_type END) AS label_type

                FROM cte
                GROUP BY created_time", goupsql, goupsql)
            }
            _ => default_sql,
        };

        let rows: Vec<Book> = sqlx::query_as::<_, Book>(&sql)
            .bind(&query.start_date)
            .bind(&query.end_date)
            .bind(query.b_id)
            // .bind(query.size)
            // .bind((query.current - 1) * query.size)
            .fetch_all(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;
        Ok(rows)
    }
    // 分页查询书本排行榜
    pub async fn query_page_paging_rank(
        query: dto::book::PagingQueryRankingDto,
    ) -> Result<ResponsPagerList<BookRank>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;
        let sort_type = match query.sort_type.as_str() {
            "monthly_ticket_ranking" => "ASC",
            "reward_ranking" => "ASC",
            _ => "DESC",
        };
        let label_query = if query.label_type.is_empty() {
            format!("LIKE '%{}%'", &query.label_type)
        } else {
            format!("= '{}'", &query.label_type)
        };
        let base_query = format!("WITH LatestBooks AS (
                SELECT 
                    *,
                    ROW_NUMBER() OVER (PARTITION BY b_id ORDER BY r_id DESC) AS rn
                FROM 
                    books
            ),
            LabelBooks AS (
                SELECT * FROM LatestBooks WHERE label_type {}
            ),
            RankedBooks AS (
                SELECT 
                    *,
                    ROW_NUMBER() OVER (ORDER BY {} {}) AS `rank`
                FROM 
                    LabelBooks
                WHERE 
                    rn = 1
            )", &label_query, &query.sort_type, sort_type).to_string();
        let mut list_sql = base_query.clone();
        list_sql.push_str(
            "
            SELECT 
                id, 
                b_id, 
                book_name, 
                book_type, 
                `rank`,
                tags, 
                like_num, 
                collect_num, 
                comment_num,
                word_count,
                finish, 
                comment_long_num, 
                DATE_FORMAT(created_time, '%Y-%m-%d') as created_time, 
                tap_num, 
                cover_url, 
                monthly_pass, 
                monthly_ticket_ranking, 
                reward_ranking, 
                DATE_FORMAT(last_update_time, '%Y-%m-%d') as last_update_time, 
                label_type
            FROM 
                RankedBooks
            WHERE 
                book_name LIKE ?
            LIMIT ? OFFSET ?;",
        );
        let rows: Vec<BookRank> = sqlx::query_as::<_, BookRank>(&list_sql)
            // .bind(format!("%{}%", query.label_type)) // Correctly bind the label_type with wildcard
            .bind(format!("%{}%", query.book_name)) // Correctly bind the book_name with wildcard
            .bind(query.size)
            .bind((query.current - 1) * query.size)
            .fetch_all(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;
        let mut total_num_query = base_query.clone();
        total_num_query.push_str(
            "SELECT 
            COUNT(*) AS total
        FROM 
            RankedBooks
        WHERE 
            book_name LIKE ?;",
        );
        let total_num: i32 = sqlx::query_scalar(&total_num_query)
            // .bind(format!("%{}%", query.label_type))
            .bind(format!("%{}%", query.book_name))
            .fetch_one(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;

        Ok(ResponsPagerList::new(ResponsPagerListFrom {
            current: query.current,
            size: query.size,
            list: rows,
            total_num: total_num as i32, // Convert to i32 for the response
        }))
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
                chrono::NaiveDate::parse_from_str(&new_record.last_update_time.clone(), "%Y-%m-%d")
                    .unwrap();
            let current_date = chrono::Local::now().date_naive();
            if current_date
                .signed_duration_since(new_record_time)
                .num_days()
                < 30 && book.finish == 0
            {
                // 本书处于维护中
                return Err(actix_web::error::ErrorBadRequest("book_state_maintenance"));
            }
            if current_date
                .signed_duration_since(lash_update_time)
                .num_days()
                > 30 || book.finish == 1
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

        let sql = "INSERT INTO books (id, b_id, book_name, cover_url, book_type, tags, like_num, collect_num, comment_num, comment_long_num, created_time, tap_num, monthly_pass, monthly_ticket_ranking, reward_ranking, last_update_time, label_type, word_count, finish) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
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
            .bind(&new_book.label_type)
            .bind(new_book.word_count)
            .bind(new_book.finish)
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
        // 分组查询最新的创建时间那条, 取最近更新时间 >= 30天的连载中作品。
        let sql = "
           SELECT b_id, MAX(created_time) as max_created_time
            FROM books
            WHERE last_update_time >= DATE_SUB(CURDATE(), INTERVAL 30 DAY) AND finish = 0
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
    // 查询所有的征文类型
    pub async fn query_all_label_types(keyword: String) -> Result<Vec<String>, actix_web::Error> {
        let client = client::connect().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database connection error: {}", e))
        })?;
        // 分组查询最新的创建时间那条, 取最近更新时间 >= 30天的。
        let sql = "
           SELECT DISTINCT label_type
            FROM books
            WHERE label_type <> '' AND label_type LIKE ?;
        ";

        let labels: Vec<String> = sqlx::query_scalar(sql)
            .bind(format!("%{}%", keyword))
            .fetch_all(&*client)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database query error: {}", e))
            })?;

        Ok(labels)
    }
    // 关键词搜索主站的书本
    pub async fn keyword_search_master_books(keyword: String) -> Result<actix_web::web::Json<Vec<serde_json::Value>>, actix_web::Error> {
        let base_head_url = env::var("SF_DATA_BASE_URL").expect("未获取到sf接口网址");
        let api_url = format!("{}/ajax/ashx/GetRelateWord.ashx?t=1", base_head_url);
        
        let form = [("keyword", keyword)];
        let response = reqwest::Client::new()
            .post(&api_url)
            .form(&form)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        if response.status().is_success() {
            let json_response: Vec<serde_json::Value> = response.json().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let re = Regex::new(r"/(\d+)").unwrap(); // Regex to extract bid from URL
            let result: Vec<serde_json::Value> = json_response.into_iter().map(|item| {
                let url = item["url"].as_str().unwrap_or("");
                let b_id = re.captures(url)
                    .and_then(|caps| caps.get(1))
                    .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
                serde_json::json!({
                    "b_id": b_id,
                    "clear_title": item["clearTitle"],
                    "title": item["title"],
                    "url": item["url"]
                })
            }).collect();
            Ok(actix_web::web::Json(result))
        } else {
            Err(actix_web::error::ErrorInternalServerError("Failed to fetch data from external API"))
        }
    }
    // 爬虫, 爬取这本书的数据
    pub async fn find_sf_book(book_id: i32) -> Result<Book, actix_web::Error> {
        let base_head_url = env::var("SF_DATA_BASE_URL").expect("未获取到sf接口网址");
        let mb_head_url = env::var("SF_MB_BASE_URL").expect("未获取到sf接口网址");
        let base_url = format!("{}{}", base_head_url.clone(), "/Novel/");
        let label_base_url = format!("{}{}", mb_head_url, "/b/");
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
        let mut label_type = String::new();
        let mut finish = 0;
        let mut word_count = 0;
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
                    } else if text.starts_with("字数：") {
                        let word_count_text = text.replace("字数：", "").trim().to_string();
                        if let Some(end_index) = word_count_text.find('[') {
                            word_count = word_count_text[..end_index].trim().parse::<i32>().unwrap_or(0);
                            finish = if word_count_text[end_index..].contains("已完结") { 1 } else { 0 };
                        } else {
                            word_count = word_count_text.parse::<i32>().unwrap_or(0);
                            finish = 0; // Default to false if not specified
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
            // 爬取移动端数据
            let label_info_url = format!("{}{}", label_base_url, book_id);
            let label_info_response = reqwest::get(&label_info_url)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            if label_info_response.status().is_success() {
                let label_info_body = label_info_response
                    .text()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                // 解析征文类型
                let document = Html::parse_document(&label_info_body);
                let selector = Selector::parse("ul.book_info").unwrap();

                if let Some(element) = document.select(&selector).next() {
                    let label_selector = Selector::parse("label").unwrap();
                    if let Some(label_element) = element.select(&label_selector).next() {
                        label_type = label_element
                            .text()
                            .collect::<Vec<_>>()
                            .join(", ")
                            .trim()
                            .to_string();
                    }
                }
            } else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "Failed to fetch label info",
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
            finish,
            word_count,
            created_time: String::new(),
            label_type: label_type.clone(), // Automatically generate the current time in YYYY-MM-DD format
        });
        Ok(new_book)
    }
}
