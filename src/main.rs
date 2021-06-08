use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_num = input.trim().parse::<i32>().expect("Failed") / 10;
    match input_num{
        10 => println!("A"),
        9 => println!("A"),
        8 => println!("B"),
        7 => println!("C"),
        6 => println!("D"),
        _ => println!("F"),
    }
}
