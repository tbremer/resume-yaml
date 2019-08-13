mod structs;

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Resume {
    basics: structs::General,
    work: Vec<structs::Job>,
    education: Vec<structs::Education>,
    awards: Vec<structs::Accomplishment>,
    skills: Vec<structs::Skill>,
    languages: Vec<structs::Language>,
}

fn main() {
    let path = std::env::args().last().unwrap();
    let str = std::fs::read_to_string(path).unwrap();
    let yml_string: Resume = serde_yaml::from_str(&str).unwrap();
    let output: String = parse_resume(&yml_string);
    let mut file = File::create("resume.html").unwrap();

    file.write_all(output.as_bytes()).unwrap();

    // println!("yml_string: {:?}", yml_string);
    // println!("output: {:?}", output);
}

fn parse_resume(r: &Resume) -> String {
    let work: Vec<String> = r.work.clone().into_iter().map(|w| w.to_string()).collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{}</title>

    <link rel="stylesheet" type="text/css" href="https://unpkg.com/normalize.css@8.0.1/normalize.css">
    <link rel="stylesheet" type="text/css" href="https://rsms.me/inter/inter.css">
    <style>
    :root {{
        font-family: Inter, sans-serif;
        font-size: 100%;
        color: #16161d;
        line-height: 1.4;
    }}

    *, *::before, *::after {{
        box-sizing: border-box;
    }}

    a {{
        color: blue;
        text-decoration: none;
    }}

    a:hover {{
        color: blue;
        text-decoration: underline
    }}

    /*html, body {{ width: 100%; height: 100% }} */
    body {{
        padding: 2rem;
        padding-bottom: 0;
        max-width: 70rem;
        margin: 0 auto;
    }}

    h1, h2, h3, h4, h5, h6 {{ margin: 0; }}
    h2 {{ font-size: 1.25em; }}
    h3 {{ font-size: 1.125em; font-weight: 500; }}

    ul {{
        padding: 0;
        list-style-position: inside;
        padding-left: .5rem;
    }}

    /*li {{
        margin: .5rem 0;
    }}*/

    hr {{
        width: 33%;
        margin: .5rem 0;
    }}

    .basics {{
        display: flex;
        align-items: center;
        margin-bottom: 2rem;
    }}

    .basics .info {{ flex: 1; }}

    .basics img {{
        max-width: 11.25rem;
        border-radius: 50%;
    }}

    @media screen and (max-width: 800px) {{
        .basics img {{
            max-width: 8.75rem;
        }}
    }}

    @media screen and (max-width: 700px) {{
        body {{ padding: 1rem; padding-bottom: 0; }}
        .basics {{
            flex-direction: column;
            align-items: flex-start;
        }}

        .basics img {{
            order: -1;
            margin-bottom: 2rem;
        }}

        .info h1 {{
            font-size: 1.75rem
        }}
    }}


    .basics .info h2 {{
        margin-bottom: 1rem;
    }}

    .spaced-section {{
        /* margin-left: calc(11.25rem + 2rem); */
        margin-bottom: 2rem;
    }}

    .w-auto {{
        width: auto;
    }}

    .border-light {{ border-color: rgba(255, 255, 255, .25); }}

    .text-light {{ color: #767676; }}

    .m-y-2 {{ margin: 1rem 0; }}
    .m-0 {{ margin: 0; }}
    .m-b-1 {{ margin-bottom: 1rem; }}

    .job {{ margin-top: 1rem; }}
    .job li {{ line-height: 1.75; }}
    </style>
</head>

<body itemscope="itemscope" itemtype="http://schema.org/Person">
    {}
    <section class="spaced-section profiles">
        <h2>Work History</h2>
        <hr />
        {}
    </section>
</body>

</html>"#,
        r.basics.name,
        r.basics.to_string(),
        work.join(r#"<hr class="w-auto m-y-2 border-light"/>"#)
    )
}
