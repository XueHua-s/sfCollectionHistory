use serde::{Deserialize, Serialize};
use uuid::Uuid;
// 结构体转换为json分发特征
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: Option<String>,
    pub b_id: i32,
    pub book_name: String,
    pub book_type: String,
    pub tap_num: i32,
    pub tags: String,
    pub like_num: i32,
    pub collect_num: i32,
    pub comment_num: i32,
    pub comment_long_num: i32,
    pub created_time: String,
}

impl Book {
    // 重新生成id
    pub fn from(book: Book) -> Book {
        Book {
            id: Some(Uuid::new_v4().to_string()),
            b_id: book.b_id,
            book_name: book.book_name,
            tap_num: book.tap_num,
            book_type: book.book_type,
            tags: book.tags,
            like_num: book.like_num,
            collect_num: book.collect_num,
            comment_num: book.comment_num,
            comment_long_num: book.comment_long_num,
            created_time: book.created_time,
        }
    }
}
