use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    name: String,
    keywords: Vec<String>,
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        let keywords: Vec<String> = self
            .keywords
            .clone()
            .into_iter()
            .map(|k| format!(r#"<li>{}</li>"#, k))
            .collect();

        format!(
            r#"
        <h3>{}</h3>
        <ul>{}</ul>
        "#,
            self.name,
            keywords.join("\n")
        )
    }
}
