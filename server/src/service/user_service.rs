use crate::model::user::UserInfo;
use crate::mysql::client;
use uuid::Uuid;
use sqlx::{Error, Row};
pub struct UserService;

impl UserService {
    // 用户服务层静态方法
    pub async fn get_user_info(p_name: String) -> UserInfo {
        let my_uid = Uuid::new_v4();
        UserInfo {
            id: Some(my_uid.to_string()),
            name: p_name.clone(),
            age: 18,
            email: String::from("111@qq.com"),
        }
    }
    pub async fn add_user(user: UserInfo) -> Result<UserInfo, Error> {
        let client = client::connect().await?;
        let sql = "INSERT INTO users (id, name, age, email) VALUES (?, ?, ?, ?)";
        sqlx::query(sql)
            .bind(user.id.clone())
            .bind(&user.name)
            .bind(user.age)
            .bind(&user.email)
            .execute(&*client)
            .await?;
        Ok(user)
    }
    pub async fn find_all_users() -> Result<Vec<UserInfo>, Error> {
        let client = client::connect().await?;
        let rows = sqlx::query("SELECT * FROM users")
            .fetch_all(&*client)
            .await?;
        let users = rows.into_iter().map(|row| {
            UserInfo {
                id: row.try_get("id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                age: row.try_get("age").unwrap_or_default(), // 假设一个默认值，或从数据库中获取
                email: row.try_get("email").unwrap_or_default(), // 假设一个默认值，或从数据库中获取
            }
        }).collect();

        Ok(users)
    }
}
