use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Accomplishment {
    title: String,
    date: String,
    awarder: String,
    summary: String,
}
