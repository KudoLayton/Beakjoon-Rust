use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let input_list = input.trim().split_whitespace().map(|x| x.parse::<i32>().expect("Failed"));
    let max_iter = input_list.clone();
    let min_iter = input_list.clone();
    println!("{} {}", min_iter.min().unwrap(), max_iter.max().unwrap());
}
