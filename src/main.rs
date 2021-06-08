use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let numbers : Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
    println!("{}", numbers[0] + numbers[1]);
    println!("{}", numbers[0] - numbers[1]);
    println!("{}", numbers[0] * numbers[1]);
    println!("{}", numbers[0] / numbers[1]);
    println!("{}", numbers[0] % numbers[1]);
}
