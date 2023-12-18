use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct ResPokedexApi{
    pub weight:i64,
}