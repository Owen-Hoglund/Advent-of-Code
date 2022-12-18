use std::fs::File;
use std::io::prelude::*;
#[allow(dead_code)]
pub fn load_input(filename: &str) -> String {
    let mut file = File::open(filename).expect("Can't Open File");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Can't Read File");
    return input;
}