use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let diff = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).fold(0, |acc, x| -(acc + x));
    match diff {
        0 => println!("=="),
        _ => match diff / diff.abs() {
            1 => println!(">"),
            _ => println!("<"),
        }
    }
}