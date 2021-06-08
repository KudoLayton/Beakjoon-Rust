use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_chars: Vec<_> = input.trim().chars().collect();
    println!("{}", input_chars[0] as u8);
}
