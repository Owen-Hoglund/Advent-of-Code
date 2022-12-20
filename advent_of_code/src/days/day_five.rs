use crate::utility;

#[allow(dead_code)]
pub fn day_five(){
    let stacks: String;
    let orders: String;
    (stacks, orders) = populate_data();

    let mut stacks = fix_bad_stacking_representation(stacks);
    let orders = order_processing(orders);
    // println!("{:?}", stacks);
    // println!("{:?}", orders);

    process_orders(&mut stacks, orders);
    
    for mut stack in stacks {
        println!("{}", stack.pop().unwrap());
    }


}

fn process_orders(stacks: &mut Vec<Vec<char>>, orders: Vec<Vec<i32>>){
    for order in orders {
        // execute_order_version_one(stacks, order);
        execute_order_version_two(stacks, order);
        println!("{:?}", stacks);

    }
}
#[allow(dead_code)]
fn execute_order_version_one(stacks: &mut Vec<Vec<char>>, orders: Vec<i32>){
    println!("Moving {} boxes from position {} to position {}", orders[0], orders[1], orders[2]);
    for _ in 0..orders[0]{
        let a = stacks[(orders[1] - 1) as usize].pop().unwrap();
        stacks[(orders[2] - 1) as usize].push(a);
    }
}
fn execute_order_version_two(stacks: &mut Vec<Vec<char>>, orders: Vec<i32>){
    println!("Moving {} boxes from position {} to position {}", orders[0], orders[1], orders[2]);
    // I wanted to grab slices and move them but that actually turns out to be very annoying, so instead we are going to make an intermediate stack
    let mut a: Vec<char> = Vec::new();
    for _ in 0..orders[0]{
        a.push(stacks[(orders[1] - 1) as usize].pop().unwrap());
    }
    for _ in 0..orders[0]{
        stacks[(orders[2] - 1) as usize].push(a.pop().unwrap());
    }

}

fn populate_data() -> (String,String){
    let  data = utility::load_input("five");
    let mut data = data.split("\r\n\r\n");
    (data.next().unwrap().to_string(), data.next().unwrap().to_string())
}


fn fix_bad_stacking_representation(stack:String) -> Vec<Vec<char>>{
    // The way they represented the stacks of crates in this problem is completely nonsensical, this fixes it, and adapts to other sizes if necessary. 
    let stack= stack.split("1").collect::<Vec<&str>>();
    let stack = stack[0].split("\r\n").collect::<Vec<&str>>().iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut result: Vec<Vec<char>> = Vec::new();
    for _i in 0..(stack[0].len() + 1)/4 {
        result.push(Vec::new());
    }

    for line in stack.iter(){
        for (x, c) in line.iter().enumerate(){
            if x % 4 == 1  && *c != ' '{
                result[x / 4].push(*c);
            }
        }
    }
    result.iter_mut().for_each(|x| x.reverse());
    return result;
}


fn order_processing(orders: String) -> Vec<Vec<i32>>{
    let orders = orders.split("\r\n").collect::<Vec<&str>>();
    let orders = orders.into_iter().map(|x| x.split_whitespace()).flatten().collect::<Vec<&str>>();
    let orders = orders.into_iter().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    orders.chunks(3).map(|x| x.to_vec()).collect()
}

