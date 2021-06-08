use std::io;

fn main() {
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Failed");
  let v = a.split_whitespace();
  print!("{}", v.count());
}