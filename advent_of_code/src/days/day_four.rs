use crate::utility;

pub fn day_four(){
    let input = utility::load_input(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\Advent-of-Code\advent_of_code\src\input_files\day_four_input.txt")
    let ranges = pairing(input);
    count(ranges);
}

fn count(ranges: Vec<Vec<i32>>){
    let pairs:Vec<Vec<Vec<i32>>> = ranges.chunks(2).map(|x| x.to_vec()).collect();
    let mut subset_count = 0;
    let mut intersection_count = 0;
    for pair in pairs{
        let l = (&pair[0][0], &pair[0][1]);
        let r = (&pair[1][0], &pair[1][1]);
        if is_subset(l, r){
            subset_count += 1;
        }
        if intersects(l, r){
            intersection_count += 1;
        }
    }

    println!("Subset Count: {}\nIntersection Count: {}", subset_count, intersection_count);
}

// Simple boundary Checking
fn is_subset(A: (&i32, &i32), B: (&i32, &i32)) -> bool{
    A.0 <= B.0 && A.1 >= B.1 || A.0 >= B.0 && A.1 <= B.1
} 

// This is definitely not as efficient as just checking a few boundary conditions but I wanted to try a different approach
fn intersects(A: (&i32, &i32), B: (&i32, &i32)) -> bool {
    (*A.0..=*A.1).filter(|n| (*B.0..= *B.1).contains(n)).count() > 0
}


fn pairing(input:String) -> Vec<Vec<i32>>{
    let lines: Vec<i32> = input.split("\r\n").map(|x|{
        x.split(&[',', '-']).collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>().into_iter().flatten().map(|x| x.parse().unwrap()).collect();
    let result:Vec<Vec<i32>> = lines.chunks(2).map(|x| x.to_vec()).collect();
    return result;
}