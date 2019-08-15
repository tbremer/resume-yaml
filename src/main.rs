mod arg_parser;
mod parser;
mod structs;

use serde_yaml;
use std::{fs::File, io::prelude::*};

fn main() {
    let args = arg_parser::parse();
    let str = match args.get("path") {
        Some(p) => std::fs::read_to_string(p).unwrap(),
        None => panic!("Input file not supplied"),
    };
    let yml_string: structs::Resume = serde_yaml::from_str(&str).unwrap();
    let output: String = parser::parse_resume(&yml_string);
    let file_path = match args.get("output") {
        Some(p) => p,
        None => "resume.html",
    };
    let mut file = File::create(file_path).unwrap();

    file.write_all(output.as_bytes()).unwrap();
}
