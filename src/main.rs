use std::io;

fn main() {
  let mut max_val = 0;
  let mut max_val_idx = -1;
  for i in 0..9{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_val = input.trim().parse::<i32>().expect("Failed");
    if max_val < input_val{
      max_val = input_val.clone();
      max_val_idx = i.clone();
    }
  }
  println!("{}", max_val);
  println!("{}", max_val_idx + 1);
}