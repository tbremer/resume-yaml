use std::{collections::HashMap, env::args, process::exit};
type ArgHash<'a> = HashMap<&'a str, String>;

pub fn parse<'a>() -> ArgHash<'a> {
    let exe = match args().nth(0) {
        Some(p) => p,
        None => panic!("Could not get first argument"),
    };
    let path = match args().last() {
        Some(p) => p,
        None => panic!("At a minimum the path to the YAML file needs to be passed in"),
    };
    let mut passed_args = args();
    let mut args = HashMap::new();

    args.insert("path", path.clone());

    loop {
        if let Some(cur_arg) = passed_args.next() {
            let arg_ref = cur_arg.as_ref();
            if arg_ref == path {
                continue;
            }

            match arg_ref {
                "--help" | "-h" => {
                    println!("display help");
                    exit(0)
                }
                "--output" | "-o" => match passed_args.next() {
                    Some(path) => {
                        args.insert("output", path);
                    }
                    None => panic!("Output called without path."),
                },
                unknown => {
                    if unknown != exe {
                        println!("Unknonw arg: {}", unknown);
                        exit(128)
                    }
                }
            };
        } else {
            break;
        }
    }

    args
}
