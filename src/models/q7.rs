use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ResBakeCookies{
    pub cookies:i64,
    pub pantry:Value,
}