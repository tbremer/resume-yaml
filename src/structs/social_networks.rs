use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Network {
    GitHub,
    Twitter,
    CodePen,
}

impl Network {
    pub fn to_url(&self) -> String {
        match self {
            Network::GitHub => String::from("https://github.com/"),
            Network::Twitter => String::from("https://twitter.com/"),
            Network::CodePen => String::from("https://codepen.io/"),
        }
    }
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
            r#"<a href="{}{}">{} / {}</a>"#,
            self.network.to_url(),
            self.username,
            self.network.to_string(),
            self.username,
        )
    }
}
