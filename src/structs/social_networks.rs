use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Network {
    GitHub,
    Twitter,
    CodePen,
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::GitHub => String::from("github"),
            Network::Twitter => String::from("twitter"),
            Network::CodePen => String::from("codepen"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    network: Network,
    username: String,
}

impl ToString for Profile {
    fn to_string(&self) -> String {
        format!(
            r#"<a href="https://{}.com/{}">{} / {}</a>"#,
            self.network.to_string(),
            self.username,
            self.network.to_string(),
            self.username
        )
    }
}
