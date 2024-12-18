use crate::dto::book::{PageQueryBookAnalysisRecordsReq, PagingQueryRankingDto};
use crate::model::response;
use crate::service::book_services::BookServices;
use actix_web::{get, post, web, HttpResponse, Responder};
// 收录书本
#[post("/add/{book_id}")]
async fn add_book(book_id: web::Path<i32>) -> impl Responder {
    match BookServices::add_sf_book(*book_id).await {
        // Dereference book_id to get the i32 value
        Ok(book) => HttpResponse::Ok().json(response::ResponseOk::new(book)), // 成功时返回 JSON 响应
        Err(err) => {
            if err.to_string() == "has_book" {
                HttpResponse::Ok().json(response::ResponseMsg::new("此书已被收录".to_string(), err.to_string()))
            } else {
                HttpResponse::InternalServerError()
                    .json(response::ResponseError::new(err.to_string()))
            }
        } // 错误时返回错误信息
    }
}
// 查询维护书本
#[get("/all/bid")]
async fn get_all_bid() -> impl Responder {
    match BookServices::find_sf_all_bid().await {
        Ok(bids) => HttpResponse::Ok().json(response::ResponseOk::new(bids)), // 成功时返回 JSON 响应
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
        } // 错误时返回错误信息
    }
}
// 书本详情
#[get("/detail/{book_id}")]
async fn get_book_detail(book_id: web::Path<i32>) -> impl Responder {
    match BookServices::get_book_new_once_detail(*book_id).await {
        Ok(book) => HttpResponse::Ok().json(response::ResponseOk::new(book)), // 成功时返回 JSON 响应
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
        } // 错误时返回错误信息
    }
}
// 时间条件查询书本数据记录
#[post("/analysis/records")]
async fn query_book_analysis_records(
    req: web::Json<PageQueryBookAnalysisRecordsReq>,
) -> impl Responder {
    match PageQueryBookAnalysisRecordsReq::validate_req(req.into_inner()) {
        Ok(query) => match BookServices::page_query_book_analysis_records(query).await {
            Ok(book) => HttpResponse::Ok().json(response::ResponseOk::new(book)), // 成功时返回 JSON 响应
            Err(err) => {
                HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
            } // 错误时返回错误信息
        },
        Err(validation_err) => {
            HttpResponse::BadRequest().json(response::ResponseError::new(validation_err))
        } // 返回请求验证错误信息
    }
}
// 查询书本排行记录
#[post("/rank/records")]
async fn paging_query_ranking (req: web::Json<PagingQueryRankingDto>) -> impl Responder {
    match PagingQueryRankingDto::validate_req(req.into_inner()) {
        Ok(query) => match BookServices::query_page_paging_rank(query).await {
            Ok(ranks_list) => HttpResponse::Ok().json(response::ResponseOk::new(ranks_list)), // 成功时返回 JSON 响应
            Err(err) => {
                HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
            } // 错误时返回错误信息
        },
        Err(validation_err) => {
            HttpResponse::BadRequest().json(response::ResponseError::new(validation_err))
        } // 返回请求验证错误信息
    }
}
// 恢复维护
#[post("/maintenance/{book_id}")]
async fn to_book_maintenance (book_id: web::Path<i32>) -> impl Responder {
    match BookServices::to_book_maintenance(*book_id).await {
        // Dereference book_id to get the i32 value
        Ok(book) => HttpResponse::Ok().json(response::ResponseOk::new(book)), // 成功时返回 JSON 响应
        Err(err) => {
            if err.to_string() == "not_has_book" {
                HttpResponse::Ok().json(response::ResponseMsg::new("未收录, 请先提交收录。".to_string(), err.to_string()))
            } else if err.to_string() == "book_state_maintenance" {
                HttpResponse::Ok().json(response::ResponseMsg::new("该书处于维护中。".to_string(), err.to_string()))
            } else if err.to_string() == "maintenance_max" {
                HttpResponse::Ok().json(response::ResponseMsg::new("已太监, 不维护, 请先恢复正常更新。".to_string(), err.to_string()))
            } else {
                HttpResponse::InternalServerError()
                    .json(response::ResponseError::new(err.to_string()))
            }
        } // 错误时返回错误信息
    }
}
// 配置方法：将所有路由绑定到 App
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .service(add_book)
            .service(get_all_bid)
            .service(get_book_detail)
            .service(query_book_analysis_records)
            .service(to_book_maintenance)
            .service(paging_query_ranking)
    ); // 默认路由设置为 /user
}
