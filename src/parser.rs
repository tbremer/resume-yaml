use super::structs::Resume;
use std::string::ToString;

fn build_meta(r: &Resume) -> String {
    format!(
        r#"<meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <meta name="description" content="{}">
    <meta name="image" content="{}">

    <meta property="og:url" content="https://tbremer.com/">
    <meta property="og:title" content="{}">
    <meta property="og:description" content="{}">
    <meta property="og:image" content="{}">

    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:creator" content="_tbremer">
    <meta name="twitter:title" content="{}">
    <meta name="twitter:description" content="{}">
    <meta name="twitter:image" content="{}">"#,
        r.basics.summary,
        r.basics.picture,
        r.basics.name,
        r.basics.summary,
        r.basics.picture,
        r.basics.name,
        r.basics.summary,
        r.basics.picture,
    )
}

fn collect<T: ToString>(item: &Option<Vec<T>>) -> Vec<String> {
    match &item {
        None => vec![],
        Some(work) => work.clone().into_iter().map(|w| w.to_string()).collect(),
    }
}

pub fn parse_resume(r: &Resume) -> String {
    let work = collect(&r.work);
    let skills = collect(&r.skills);
    let education = collect(&r.education);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">

<head>
    {}

    <title>{} Resume</title>

    <style>
    :root {{
        font-size: 100%;
			font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
				'Open Sans', 'Helvetica Neue', sans-serif;
			color: #191e19;
			line-height: 1.4;
    }}

    *, *::before, *::after {{
        box-sizing: border-box;
    }}

    a {{
        color: #805AD5;
        text-decoration: none;
    }}

    a:hover {{
        text-decoration: underline
    }}

    body {{
        background-color: #FAF5FF;
        padding: 2rem;
        max-width: 70rem;
        margin: 0 auto;
    }}

    main {{
        background-color: #fff;
        padding: 2rem;
        border-radius: 0.25rem;
        box-shadow: 0 0.25rem 0.375rem -0.0625rem rgba(0, 0, 0, 0.1),
            0 0.125rem 0.25rem -0.0625rem rgba(0, 0, 0, 0.06);
    }}

    h1, h2, h3, h4, h5, h6 {{ font-weight: bold; margin: 0; }}
    h1 {{ font-size: 2em; margin: 0.5rem 0; }}
    h2 {{ font-size: 1.25em; }}
    h3 {{ font-size: 1.125em; font-weight: 500; }}

    ul {{
        padding: 0;
        list-style: inside disc;
        padding-left: .5rem;
    }}

    hr {{
        width: 33%;
        min-width: 15rem;
        margin: .5rem 0;
        border-style: inset;
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
        margin-bottom: 3rem;
    }}

    .spaced-section:last-of-type {{ margin-bottom: 0; }}

    .w-auto {{
        width: auto;
    }}

    .border-light {{ border-color: rgba(255, 255, 255, .25); }}

    .text-light {{ color: #767676; }}

    .m-y-2 {{ margin: 1rem 0; }}
    .m-0 {{ margin: 0; }}
    .m-b-0 {{ margin-bottom: 0; }}
    .m-b-1 {{ margin-bottom: 1rem; }}

    .job {{ margin-top: 1rem; }}
    .job li {{ line-height: 1.75; }}

    .skills {{}}
    .skills ul {{
        margin-left: -.25rem;
        padding: 0;
        list-style: none;
        display: flex;
        align-items: center;
        flex-wrap: wrap;
    }}
    .skills li {{
        flex-shrink: 0;
        background-color: #805AD5;
        color: #FFF5F7;
        margin: .25rem;
        padding: .25rem .75rem;
        border-radius: 1.5rem;
        font-size: .75rem;
    }}

    /* print-only */
    .print-only {{ display: none; }}
    @media print {{
        li.print-only {{ display: list-item; }}
    }}

    @media print {{
        :root {{ color: black; font-size: 90%; }}
        body {{
            background-color: #fff;
            margin: 0 auto;
        }}

        main {{
            background-color: #fff;
            padding: 0;
            box-shadow: none;
        }}
        a {{ color: black }}
        .basics {{ margin-bottom: 1rem; }}
        .basics img {{ display: none; }}
        .spaced-section {{ margin-bottom: 2rem; }}

        .info ul {{
            list-style: none;
            display: flex;
        }}
        .info li::after {{
            content: '\0020\2022\0020';
            white-space: pre;
            font-weight: 500;
        }}
        .info li:last-child::after {{ content: '' }}
        .job li {{ line-height: 1.4; }}
        .skills ul {{
            margin-left: 0;
        }}
        .skills li {{
            background-color: transparent;
            color: black;
            margin: 0;
            padding: 0;
            border-radius: 0;
            font-size: 1rem;
        }}
        .skills li::after {{ white-space:pre; content: ',\0020' }}
        .skills li:last-child::after {{ content: '.' }}
    }}

    </style>
</head>

<body itemscope="itemscope" itemtype="http://schema.org/Person">
    <main>
        {}
        <section class="spaced-section work">
            <h2>Work History</h2>
            <hr />
            {}
        </section>
        <section class="spaced-section skills">
            <h2>Skills</h2>
            <hr />
            {}
        </section>
        <section class="spaced-section education">
            <h2>Education</h2>
            <hr />
            {}
        </section>
    </main>
</body>
</html>"#,
        build_meta(&r),
        r.basics.name,
        r.basics.to_string(),
        work.join(r#"<hr class="w-auto m-y-2 border-light"/>"#),
        skills.join(r#"<hr class="w-auto m-y-2 border-light"/>"#),
        education.join(r#"<hr class="w-auto m-y-2 border-light"/>"#),
    )
}
