use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    company: String,
    position: String,
    website: String,
    #[serde(rename(serialize = "startDate", deserialize = "startDate"))]
    start_date: String,
    #[serde(rename(serialize = "endDate", deserialize = "endDate"))]
    end_date: Option<String>,
    summary: String,
    highlights: Vec<String>,
}

impl ToString for Job {
    fn to_string(&self) -> String {
        let time_at_job = match &self.end_date {
            Some(end_date) => format!("{} - {}", self.start_date, end_date),
            None => format!("{} - Present", self.start_date),
        };
        let highlights: Vec<String> = self
            .highlights
            .clone()
            .into_iter()
            .map(|h| format!(r#"<li>{}</li>"#, h))
            .collect();

        format!(
            r#"<section class="job">
            <h3><strong>{}</strong>, {}</h3>
            <p class="text-light m-0 m-b-1"><small>{}</small></p>

            <h4>{}</h4>
            <ul>
            {}
            </ul>
        </section>"#,
            self.position,
            self.company,
            time_at_job,
            self.summary,
            highlights.join("\n"),
        )
        // format!(r#"<section class="job">work</section>"#)
        // String::from("work,")
    }
}

// <div class="content">
//     <p class="clear-margin relative"><strong>Senior Front End Engineer</strong>,&nbsp;<a href="https://wwt.com" target="_blank">World Wide Technology: Application Services</a></p>
//     <p class="text-muted"><small><span class="space-right">Jul, 2015 - Present</span></small></p>
//     <div class="mop-wrapper space-bottom">
//         <p>Front End Engineer, User Experience Research.</p>
//     </div>
//     <ul>
//         <li class="mop-wrapper">
//             <p>Deploy Enterprise web applications across web and native mobile devices</p>
//         </li>
//         <li class="mop-wrapper">
//             <p>Promote web accessibility standards throughout new and existing teams</p>
//         </li>
//         <li class="mop-wrapper">
//             <p>Mentor designers and developers across a range of experience levels in JavaScript, CSS, PostCSS, Sass, HTML, and the NodeJS ecosystem</p>
//         </li>
//         <li class="mop-wrapper">
//             <p>Encourage and coach teams on best practices regarding performance tuning web applications</p>
//         </li>
//         <li class="mop-wrapper">
//             <p>Facilitate lunch and learns over new and existing web technologies</p>
//         </li>
//         <li class="mop-wrapper">
//             <p>Help incorporate latest web technologies and tools within the organization</p>
//         </li>
//     </ul>
// </div>
