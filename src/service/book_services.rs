use actix_web::Error;
use crate::model::book::Book;
pub  struct BookServices;
impl BookServices {
    pub async fn add_sf_book(book_id: i32) -> Result<Book, Error> {
        let new_book = Book {
            id: Some("你好世界".to_string()),
            b_id: book_id, // Default value, adjust as necessary
            book_name: String::from("Default Book Name"), // Default value
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
}
