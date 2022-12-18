use std::fs::File;
use std::io::prelude::*;
#[allow(dead_code)]
pub fn load_input(day: &str) -> String {
    let filename = [r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\Advent-of-Code\advent_of_code\src\input_files\day_",  day,  "_input.txt"].join("");

    let mut file = File::open(filename).expect("Can't Open File");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Can't Read File");
    return input;
}