use crate::model::response;
use crate::{model::user::UserInfo, service::user_service::UserService};
use actix_web::{get, post, web, HttpResponse, Responder};
#[get("/all")]
async fn all() -> impl Responder {
    match UserService::find_all_users().await {
        Ok(users) => HttpResponse::Ok().json(response::ResponseOk::new(users)), // 成功时返回 JSON 响应
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ResponseOk::new(err.to_string()))
        } // 错误时返回错误信息
    }
}
#[post("/add")]
async fn add_user(user: web::Json<UserInfo>) -> impl Responder {
    let user_info = user.into_inner(); // Extract UserInfo from web::Json
    let res = UserService::add_user(UserInfo::from(user_info)).await;
    match res {
        Ok(user) => HttpResponse::Ok().json(response::ResponseOk::new(user)),
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ResponseError::new(err.to_string()))
        }
    }
}
// 配置方法：将所有路由绑定到 App
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(all).service(add_user)); // 默认路由设置为 /user
}
