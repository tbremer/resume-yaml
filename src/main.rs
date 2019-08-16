mod arg_parser;
mod color_converter;
mod parser;
mod structs;

use serde_yaml;
use std::{fs::File, io::prelude::*, process::exit};

fn main() {
    let _theme_base = color_converter::hex_to_hsl("#805AD5");
    let args = arg_parser::parse();
    let str = match args.get("path") {
        Some(p) => std::fs::read_to_string(p).unwrap(),
        None => panic!("Input file not supplied"),
    };
    let yml_string: structs::Resume = serde_yaml::from_str(&str).unwrap();
    let output: String = parser::parse_resume(&yml_string);

    match args.get("output") {
        Some(file_path) => {
            let mut file = File::create(file_path).unwrap();

            file.write_all(output.as_bytes()).unwrap();
        }
        None => {
            println!("{}", output);
            exit(0)
        }
    };
}
