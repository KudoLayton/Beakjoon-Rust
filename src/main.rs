use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let rev_num = input.trim().chars().rev().collect::<String>();
    let rev_num = rev_num.split_whitespace().map(|x| x.parse::<i32>().expect("Failed"));
    let max_num = rev_num.fold(0, |acc, x| x.max(acc));
    println!("{}", max_num);
}
