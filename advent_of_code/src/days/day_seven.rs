use std::{collections::HashMap, hash::Hash};

use crate::utility;
#[allow(dead_code)]

struct Directory {
    full_path: String,
    subdirectories: HashMap<String, Option<i64>>,
    files: HashMap<String, i64>,
    size: Option<i64>,
}


pub fn day_seven(){
    let input = utility::load_input("seven");
    let input = input.split("\n").collect::<Vec<&str>>();
    // println!("{:?}", input);
    let dir = process(input);

}

fn process(input: Vec<&str>) -> HashMap<String, Directory>{
    let mut dir: HashMap<String, Directory> = HashMap::new();
    let current_dir: Directory;
    println!("{}", input.len());
    let mut input = input.iter().copied().peekable();
    
    while let Some(n) = input.peek() {
        command(input.by_ref());
    }
    return dir;
}


fn command<'a>(mut iterator: impl Iterator<Item = &'a str>, current_dir: Directory){
    match iterator.next(){
        Some(command) => {
            match command.split(" ").collect::<Vec<&str>>()[1] {
                "cd" => {
                    println!("changing directory {}", command);
                    cd(iterator.by_ref())
                },
                "ls" => {
                    println!("listing directory {}", command);
                    ls(iterator.by_ref())
                },
                _a => println!("{:?}", _a),
            }
        },
        None => (),
    }
}

fn cd<'a>(mut iterator: impl Iterator<Item = &'a str>){
    println!("change dir");
}
fn ls<'a>(mut iterator: impl Iterator<Item = &'a str>){
    println!("list dir");
}