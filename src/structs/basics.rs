use serde::{Deserialize, Serialize};
use std::string::ToString;

pub use super::social_networks::{Network, Profile};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    city: String,
    #[serde(rename(serialize = "countryCode", deserialize = "countryCode"))]
    country_code: Option<String>,
    region: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Basics {
    pub name: String,
    label: String,
    picture: String,
    email: String,
    phone: String,
    website: String,
    summary: String,
    location: Location,
    profiles: Vec<Profile>,
}

fn generate_link(protocol: &str, href: String, classes: Vec<&str>) -> String {
    format!(
        r#"<a class="{}" href="{}{}">{}</a>"#,
        classes.join(" "),
        protocol,
        href,
        href
    )
}

impl ToString for Basics {
    fn to_string(&self) -> String {
        let basics = format!(
            r#"
            <section class="info">
                <h1>{} / {}</h1>
                <address>
                    <ul role="list">
                        <li role="listitem">{}</li>
                        <li role="listitem" class="print-only">{}</li>
                        <li role="listitem">{}</li>
                    </ul>
                </address>
            </section>
            <img src="{}" />
    "#,
            self.name,
            self.label,
            // name_heading(self.picture.clone(), self.name.clone(), self.label.clone()),
            generate_link("mailto:", self.email.clone(), vec![]),
            generate_link("tel:", self.phone.clone(), vec![]),
            generate_link("", self.website.clone(), vec![]),
            self.picture,
        );

        let profiles: Vec<String> = self
            .profiles
            .clone()
            .into_iter()
            .map(|i| format!("<li>{}</li>", i.to_string()))
            .collect();

        format!(
            r#"
            <section class="basics">{}</section>
            <section class="spaced-section about">
                <h2>About</h2>
                <hr />
                <p>{}</p>
            </section>
            <section class="spaced-section profiles">
                <h2>Find Me Online</h2>
                <hr />
                <ul>
                    {}
                </ul>
            </section>"#,
            basics,
            self.summary,
            profiles.join("\n")
        )
    }
}
