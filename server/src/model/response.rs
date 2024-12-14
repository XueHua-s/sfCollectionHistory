use serde::{Deserialize, Serialize}; // 导入序列化和反序列化特征

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseOk<T>
where
    T: serde::Serialize, // 添加特征约束
{
    code: String,
    data: T,
}
impl<T> ResponseOk<T>
where
    T: serde::Serialize, // 添加特征约束 // Specify the type parameter T here
{
    pub fn new(data: T) -> Self {
        // Add return type
        ResponseOk {
            code: "success".to_string(),
            data,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMsg {
    code: String,
    message: String,
}
impl ResponseMsg {
    pub fn new(msg: String) -> Self {
        // Add return type
        ResponseMsg {
            code: "success".to_string(),
            message: msg.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    code: String,
    error: String,
}

impl ResponseError // 添加特征约束 // Specify the type parameter T here
{
    pub fn new(error: String) -> Self {
        // Add return type
        ResponseError {
            code: "error".to_string(),
            error: error.clone(),
        }
    }
}
