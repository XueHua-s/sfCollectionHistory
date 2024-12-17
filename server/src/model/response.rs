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
    msg_code: String
}
impl ResponseMsg {
    pub fn new(msg: String, status: String) -> Self {
        // Add return type
        ResponseMsg {
            code: "success".to_string(),
            message: msg,
            msg_code: status
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
// 分页响应结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsPagerList<T>
where
    T: serde::Serialize, // 添加特征约束 // Specify the type parameter T here
{
    current: i32,
    size: i32,
    list: Vec<T>,
    total_num: i32,
    total_page: i32
}
pub struct ResponsPagerListFrom<T>
where
    T: serde::Serialize, // 添加特征约束 // Specify the type parameter T here
{
    current: i32,
    size: i32,
    list: Vec<T>,
    total_num: i32,
}
impl<T> ResponsPagerList<T>
where
    T: serde::Serialize, // 添加特征约束 // Specify the type parameter T here
{
    pub fn new (pagers: ResponsPagerListFrom<T>) -> Self {
        ResponsPagerList {
            current: pagers.current,
            size: pagers.size,
            total_num: pagers.total_num,
            total_page:  (pagers.total_num as f64 / pagers.size as f64).ceil() as i32,
            list: pagers.list
        }
    }
}