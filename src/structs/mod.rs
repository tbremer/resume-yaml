mod awards;
mod basics;
mod education;
mod language;
mod skills;
mod social_networks;
mod work;

pub use awards::Accomplishment;
pub use basics::{Basics as General, Location};
pub use education::Schooling as Education;
pub use language::{Ability, Fluency, Language};
pub use skills::Skill;
pub use work::Job;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    pub basics: General,
    pub work: Vec<Job>,
    pub education: Vec<Education>,
    pub awards: Vec<Accomplishment>,
    pub skills: Vec<Skill>,
    pub languages: Vec<Language>,
}
