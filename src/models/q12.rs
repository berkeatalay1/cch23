use serde::Serialize;

#[derive(Serialize)]
pub struct ResUlit{
    #[serde(rename(serialize = "christmas eve"))]
    pub christmas:u32,
    #[serde(rename(serialize = "weekday"))]
    pub weekday:u32,
    #[serde(rename(serialize = "LSB is 1"))]
    pub lsb:u32,
    #[serde(rename(serialize = "in the future"))]
    pub future:u32,
}