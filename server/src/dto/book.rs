use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageQueryBookAnalysisRecordsReq {
    pub start_date: String,
    pub end_date: String,
    pub b_id: i32,
    pub current: i32, // Use NonZeroI32 to prevent zero values
    pub size: i32,
    pub group_type: Option<i32>   // Use NonZeroI32 to prevent zero values
}
impl PageQueryBookAnalysisRecordsReq {
    pub fn validate_req(
        req: PageQueryBookAnalysisRecordsReq,
    ) -> Result<PageQueryBookAnalysisRecordsReq, String> {
        let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
            .map_err(|_| "Invalid start date format".to_string())?;
        let end_date = NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
            .map_err(|_| "Invalid end date format".to_string())?;

        // Check if the date range exceeds one year
        if (end_date - start_date).num_days() > 375 {
            return Err("Date range exceeds one year".to_string());
        }

        // Format dates to YYYY-MM-DD
        let formatted_start_date = start_date.format("%Y-%m-%d").to_string();
        let formatted_end_date = end_date.format("%Y-%m-%d").to_string();

        // Validate group_type
        let group_type = match req.group_type {
            Some(value) if value == 1 || value == 2 || value == 3 => Some(value),
            Some(_) => return Err("group_type must be 1, 2, 3, or None".to_string()),
            None => None,
        };

        Ok(PageQueryBookAnalysisRecordsReq {
            start_date: formatted_start_date, // Use formatted start date
            end_date: formatted_end_date,     // Use formatted end date
            current: req.current,
            size: req.size,
            b_id: req.b_id,
            group_type,
        })
    }
}
