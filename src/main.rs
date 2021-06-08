use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let left = input.trim().parse::<i32>().expect("Failed");

    for right in 1..10{
        println!("{} * {} = {}", left, right, left * right);
    }
}
