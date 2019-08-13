use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Degree {
    Ged,
    Diploma,
    Certificate,
    Associate,
    Bachelor,
    Master,
    Doctoral,
    Professional(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schooling {
    institution: String,
    area: String,
    #[serde(rename(serialize = "studyType", deserialize = "studyType"))]
    degree: Degree,
    #[serde(rename(serialize = "startDate", deserialize = "startDate"))]
    start_date: String,
    #[serde(rename(serialize = "endDate", deserialize = "endDate"))]
    end_date: Option<String>,
}
