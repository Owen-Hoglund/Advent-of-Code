use std::collections::HashMap;

use crate::utility;

#[allow(dead_code)]
pub fn day_three(){
    let alpha_vals = alpha_val();
    let rucksacks = partition(utility::load_input(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\Advent-of-Code\advent_of_code\src\input_files\day_three_input.txt"));
    let groups = group_sacks(rucksacks.clone());
    let sum = priority_sum_total(rucksacks, &alpha_vals);
    let badge_sum = badge_sum(groups, &alpha_vals);
    println!("Sum of all Rucksacks {}", sum);
    println!("Sum of Badge priorities {}", badge_sum);

}

fn group_sacks(sacks: Vec<Vec<String>>) -> Vec<Vec<Vec<String>>>{
    let result = sacks.chunks(3).map(|x| x.to_vec()).collect();
    return result;
}

fn alpha_val() -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    let letters = ('a'..='z').chain('A'..='Z');
    for (a, b) in letters.zip(1..=52){
        map.insert(a, b);
    }
    return map;
}
fn partition(sacks: String) -> Vec<Vec<String>>{
    let a = sacks.lines().map(|x| compartmentalize(x.to_string())).collect::<Vec<Vec<String>>>();
    return a;
}

fn compartmentalize(sack: String) -> Vec<String>{
    let x = sack.clone().chars().count()/2;
    return vec!(sack[0..x].to_string(), sack[x..].to_string());
}

fn priority_sum_total(rucksacks: Vec<Vec<String>>, alpha_val: &HashMap<char, i32>) -> i32{
    let mut sum = 0;
    for sack in rucksacks {
        let x = sack[0].chars().filter(|x| sack[1].contains(*x)).collect::<Vec<char>>();
        match alpha_val.get(&x[0]){
            Some(v) => sum += v,
            _ => panic!("No match in hashmap"),
        }
    }
    return sum;
}

fn badge_sum(groups: Vec<Vec<Vec<String>>>, alpha_val: &HashMap<char, i32>) -> i32 {
    let mut sum = 0;
    for group in groups {   
        let a = &group[0].join("");
        let b = &group[1].join("");
        let c = &group[2].join("").chars().filter(|x| a.contains(*x) && b.contains(*x)).collect::<Vec<char>>();
        match alpha_val.get(&c[0]){
            Some(v) => sum += v,
            _ => panic!("No match in hashmap"),
        }
    }
    return sum;
}