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
    pub work: Option<Vec<Job>>,
    pub education: Option<Vec<Education>>,
    pub awards: Option<Vec<Accomplishment>>,
    pub skills: Option<Vec<Skill>>,
    pub languages: Option<Vec<Language>>,
}
