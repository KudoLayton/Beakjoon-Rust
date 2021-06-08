use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed");
  let sqr_sum = input.split_whitespace().fold(0, |acc, x| acc + (x.parse::<i32>().expect("Failed").pow(2)));
  println!("{}", sqr_sum % 10);
}