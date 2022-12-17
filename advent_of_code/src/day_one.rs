use std::fs::File;
use std::io::prelude::*;
#[allow(dead_code)]
pub fn calorie_counting(){
    let mut elf_file = File::open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\Advent-of-Code\advent_of_code\src\calories_per_elf.txt").expect("Can't open file");
    let mut elves = String::new();
    elf_file.read_to_string(&mut elves).expect("Can't Read File");

    let numbers = elves.split("\r\n\r\n").map(
            |x| x.split("\r\n").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().sum()
        )
        .collect::<Vec<i32>>();
    let mut max: Vec<i32> = vec![0,0,0];
    for i in numbers {
        if i > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = i;
        } else if i > max[1] {
            max[2] = max[1];
            max[1] = i;
        } else if i > max[2] {
            max[2] = i;
        }
    }
    println!("Most Carried: {}", max[0]);
    println!("Top 3 Carrying: {}", max.iter().sum::<i32>());

}