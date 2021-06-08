use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let nums: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
    println!("{}", nums[0] * nums[1]);
}
