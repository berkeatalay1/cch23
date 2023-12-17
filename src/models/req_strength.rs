use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReqStrength{
    pub name: String,
    pub strength: i32
}