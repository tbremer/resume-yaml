use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    name: String,
    keywords: Vec<String>,
}
