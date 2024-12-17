use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
// 查询数据分析用的参数dto
pub struct PageQueryBookAnalysisRecordsReq {
    pub start_date: String,
    pub end_date: String,
    pub b_id: i32,
    // pub current: i32, // Use NonZeroI32 to prevent zero values
    // pub size: i32,
    pub group_type: Option<i32>   // Use NonZeroI32 to prevent zero values
}
impl PageQueryBookAnalysisRecordsReq {
    pub fn validate_req(
        req: PageQueryBookAnalysisRecordsReq,
    ) -> Result<PageQueryBookAnalysisRecordsReq, String> {
        let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
            .map_err(|_| "无效的开始日期格式".to_string())?;
        let end_date = NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
            .map_err(|_| "无效的结束日期格式".to_string())?;

        // 检查日期范围是否超过基于 group_type 的允许限制
        let max_days = match req.group_type {
            Some(3) => 3 * 365, // 如果 group_type 为 3，允许最多 3 年
            _ => 365, // 否则，限制为 365 天
        };

        if (end_date - start_date).num_days() > max_days {
            return Err(format!("日期范围超过 {} 天", max_days));
        }

        // 将日期格式化为 YYYY-MM-DD
        let formatted_start_date = start_date.format("%Y-%m-%d").to_string();
        let formatted_end_date = end_date.format("%Y-%m-%d").to_string();

        // 验证 group_type
        let group_type = match req.group_type {
            Some(value) if value == 1 || value == 2 || value == 3 => Some(value),
            Some(_) => return Err("group_type 必须是 1、2、3 或 None".to_string()),
            None => None,
        };

        Ok(PageQueryBookAnalysisRecordsReq {
            start_date: formatted_start_date, // 使用格式化的开始日期
            end_date: formatted_end_date,     // 使用格式化的结束日期
            // current: req.current,
            // size: req.size,
            b_id: req.b_id,
            group_type,
        })
    }
}
// 分页查询排名榜用的dto
#[derive(Serialize, Deserialize)]
pub struct PagingQueryRankingDto {
    pub current: i32,
    pub size: i32,
    pub book_name: String,
    pub sort_type: String,
    pub label_type: String,
}
impl PagingQueryRankingDto {
    pub fn validate_req(
        req: PagingQueryRankingDto,
    ) -> Result<PagingQueryRankingDto, String> {
        let valid_sort_types = vec![
            "like_num",
            "collect_num",
            "comment_num",
            "comment_long_num",
            "tap_num",
            "monthly_pass",
            "monthly_ticket_ranking",
            "reward_ranking",
        ];
        if !valid_sort_types.contains(&req.sort_type.as_str()) {
            return Err("sort_type 必须是 like_num、collect_num、comment_num、comment_long_num、tap_num、monthly_pass、monthly_ticket_ranking 或 reward_ranking".to_string());
        }
        Ok(req)
    }
}
