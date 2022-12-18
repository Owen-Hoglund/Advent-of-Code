use crate::utility;

pub fn day_six(){
    let input = utility::load_input("six");
    print!("{}", chars_to_marker(input));
}

fn chars_to_marker(buffer: String) -> i32{
    let mut marker = Vec::new();
    for (i, c) in buffer.chars().enumerate(){
        marker.insert(0, c);
        if i > 13{ // Put num of necessary unique characters to catch a marker -1 here
            marker.pop();
            if is_unique(&marker){
                return (i + 1) as i32;
            }
        }
    }
    panic!("unreachable!!!");
}

fn is_unique(marker: &Vec<char>) -> bool{
    for c in marker{
        let a = marker.iter().filter(|x| *x == c).count();
        if a > 1 {return false;}
    }
    return true;
}