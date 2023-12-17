use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqStrength{
    pub name: String,
    pub strength: i32
}


#[derive(Deserialize,Default,Clone)]
pub struct ReqContest{
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power:i32,
    pub favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    pub candies_eaten: i32
}

#[derive(Serialize)]
pub struct ResContest{
    pub fastest:String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}