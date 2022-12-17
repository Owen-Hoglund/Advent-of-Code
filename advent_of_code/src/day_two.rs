use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn day_two(){
    let mut strategy_guide = File::open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\Advent-of-Code\advent_of_code\src\strategy_guide.txt").expect("cant open file");
    let mut guide = String::new();
    strategy_guide.read_to_string(&mut guide).expect("Can't Read File");

    let score_one = guide.split("\r\n").map(|x| strategy_one(x)).collect::<Vec<i32>>();
    let final_score_one: i32 = score_one.iter().sum();

    let score_two = guide.split("\r\n").map(|x| strategy_two(x)).collect::<Vec<i32>>();
    let final_score_two: i32 = score_two.iter().sum();
    
    println!("Score under Strategy One:{:?}", final_score_one);
    println!("Score under Strategy Two:{:?}", final_score_two);

}

fn strategy_one(s: &str) -> i32{
    let s: Vec<&str> = s.split(" ").collect();
    let opp = convert(s[0]);
    let player = convert(s[1]);
    return value(player) + winner(opp, player)
}

fn strategy_two(s: &str) -> i32{
    let s: Vec<&str> = s.split(" ").collect();
    let opp = convert(s[0]);
    let player = strategy_completion(s[0], s[1]);
    return value(player) + winner(opp, player)
}

fn convert(t: &str) -> &str{
    if t == "A" || t == "X" {
        return "Rock";
    } else if t == "B" || t == "Y" {
        return "Paper"
    } else {
        return "Scissors";
    }
}
fn convert_outcome(t: &str) -> &str{
    if t == "X" {
        return "Lose";
    } else if t == "Y" {
        return "Draw"
    } else {
        return "Win";
    }
}

fn strategy_completion(opp: &str, outcome: &str) -> &'static str {
    match convert_outcome(outcome) {
        "Win" => match convert(opp) {
            "Rock" =>  return "Paper",
            "Paper" => return "Scissors",
            "Scissors" => return "Rock",
            _ => panic!("Oh god something went wrong"),
        },
        "Lose" => match convert(opp) {
            "Rock" =>  return "Scissors",
            "Paper" => return "Rock",
            "Scissors" => return "Paper",
            _ => panic!("Oh god something went wrong"),
        },
        "Draw" => match convert(opp) {
            "Rock" =>  return "Rock",
            "Paper" => return "Paper",
            "Scissors" => return "Scissors",
            _ => panic!("Oh god something went wrong"),
        },
        _ => panic!("Some secret 4th outcome??")
    }
}

fn value(p: &str) -> i32 {
    match p {
        "Rock" => return 1,
        "Paper" => return 2,
        "Scissors" => return 3,
        _ => panic!("Something has gone horribly awry"),
    }
}

fn winner(opp: &str, player: &str) -> i32{
    match [opp, player] {
        ["Rock", "Rock"] => return 3,
        ["Rock", "Paper"] => return 6,
        ["Rock", "Scissors"] => return 0,

        ["Paper", "Rock"] => return 0,
        ["Paper", "Paper"] => return 3,
        ["Paper", "Scissors"] => return 6,

        ["Scissors", "Rock"] => return 6,
        ["Scissors", "Paper"] => return 0,
        ["Scissors", "Scissors"] => return 3,
        _ => panic!("This shouldnt be possible"),
    }
}

