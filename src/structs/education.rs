use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

fn date_to_string(d: &str) -> String {
    let date = NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap();
    let month = match date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        m => panic!("Month not in range, expected 1 - 12, got: {}", m),
    };

    format!("{}, {}", month, date.year())
}

impl ToString for Schooling {
    fn to_string(&self) -> String {
        let degree = match &self.degree {
            Degree::Ged => "GED".to_string(),
            Degree::Diploma => "General Diploma".to_string(),
            Degree::Certificate => "Certificate".to_string(),
            Degree::Associate => "Associate".to_string(),
            Degree::Bachelor => "Bachelor".to_string(),
            Degree::Master => "Master".to_string(),
            Degree::Doctoral => "Doctoral".to_string(),
            Degree::Professional(val) => val.clone(),
        };
        let date_range = match &self.end_date {
            Some(end) => format!(
                "{} - {}",
                date_to_string(&self.start_date),
                date_to_string(end)
            ),
            None => format!("{} - Present", date_to_string(&self.start_date)),
        };

        let str = format!(
            r#"
        <p class="m-b-0"><strong>{}, {},</strong> {}</p>
        <p class="text-light m-0"><small>{}</small></p>
        "#,
            self.area, degree, self.institution, date_range,
        );

        str
    }
}
