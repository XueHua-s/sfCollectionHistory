use serde::{Deserialize, Serialize};
use uuid::Uuid;
// 结构体转换为json分发特征
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: Option<String>,
    pub name: String,
    pub age: i32,
    pub email: String,
}
impl UserInfo {
    // 重新生成id
    pub fn from (user: UserInfo) -> UserInfo {
        UserInfo {
            id: Some(Uuid::new_v4().to_string()),
            name: user.name,
            age: user.age,
            email: user.email
        }
    }
}
