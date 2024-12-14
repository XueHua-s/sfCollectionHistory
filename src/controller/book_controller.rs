use actix_web::{post, get, web, HttpResponse, Responder};
use crate::model::response;
use crate::service::book_services::BookServices;
#[post("/add/{book_id}")]
async fn add_book(book_id: web::Path<i32>) -> impl Responder {
    match BookServices::add_sf_book(*book_id).await { // Dereference book_id to get the i32 value
        Ok(book) => HttpResponse::Ok().json(response::ResponseOk::new(book)), // 成功时返回 JSON 响应
        Err(err) => {
            if err.to_string() == "has_book" {
                HttpResponse::Ok().json(response::ResponseMsg::new("此书已被收录".to_string()))
            } else {
                HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
            }
        } // 错误时返回错误信息
    }
}
#[get("/all/bid")]
async fn get_all_bid() -> impl Responder {
    match BookServices::find_sf_all_bid().await {
        Ok(bids) => HttpResponse::Ok().json(response::ResponseOk::new(bids)), // 成功时返回 JSON 响应
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
        } // 错误时返回错误信息
    }
}

// 配置方法：将所有路由绑定到 App
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/books").service(add_book).service(get_all_bid)); // 默认路由设置为 /user
}
