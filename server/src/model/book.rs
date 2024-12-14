use serde::{Deserialize, Serialize};
use uuid::Uuid;
// 书本基本信息
pub struct BasicBook {
    pub id: String,
    pub book_name: String,
    pub cover_url: String,
    pub book_type: String,
    pub tags: String,
    pub update_time: String,
}
// 结构体转换为json分发特征
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: Option<String>,
    pub b_id: i32,
    pub book_name: String,
    pub cover_url: String,
    pub book_type: String,
    pub tap_num: i32,
    pub tags: String,
    pub like_num: i32,
    pub collect_num: i32,
    pub comment_num: i32,
    pub comment_long_num: i32,
    pub monthly_pass: i32,
    pub monthly_ticket_ranking: i32,
    pub reward_ranking: i32,
    pub created_time: String,
}

impl Book {
    // 重新生成id
    pub fn from(book: Book) -> Book {
        Book {
            id: Some(Uuid::new_v4().to_string()),
            cover_url: book.cover_url,
            b_id: book.b_id,
            book_name: book.book_name,
            tap_num: book.tap_num,
            book_type: book.book_type,
            tags: book.tags,
            like_num: book.like_num,
            collect_num: book.collect_num,
            comment_num: book.comment_num,
            comment_long_num: book.comment_long_num,
            monthly_pass: book.reward_ranking,
            monthly_ticket_ranking: book.monthly_ticket_ranking,
            reward_ranking: book.reward_ranking,
            created_time: book.created_time,
        }
    }
    pub fn clone(&self) -> Book {
        Book {
            id: self.id.clone(),
            b_id: self.b_id,
            book_name: self.book_name.clone(),
            cover_url: self.cover_url.clone(),
            book_type: self.book_type.clone(),
            tap_num: self.tap_num,
            tags: self.tags.clone(),
            like_num: self.like_num,
            collect_num: self.collect_num,
            comment_num: self.comment_num,
            comment_long_num: self.comment_long_num,
            monthly_pass: self.monthly_pass,
            monthly_ticket_ranking: self.monthly_ticket_ranking,
            reward_ranking: self.reward_ranking,
            created_time: self.created_time.clone(),
        }
    }
    pub fn get_basic(&self) -> BasicBook {
        BasicBook {
            id: self.id.clone().unwrap_or(String::from("你好")), // Extract the value from Option, defaulting to an empty string if None
            book_name: self.book_name.clone(),
            cover_url: self.cover_url.clone(),
            book_type: self.book_type.clone(),
            tags: self.tags.clone(),
            update_time: self.created_time.clone(),
        }
    }
}
