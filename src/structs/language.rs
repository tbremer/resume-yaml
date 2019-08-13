use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SignAbility {
    Perform,
    Comprehend,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Ability {
    Written,
    Spoken,
    Sign(Vec<SignAbility>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Fluency {
    Native,
    Fluent,
    Excellent,
    Good,
    Basic,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    language: String,
    fluency: Fluency,
    abilities: Vec<Ability>,
}
