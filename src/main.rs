use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let rep = input.trim().parse::<i32>().expect("Failed");

    for i in 0..rep{
        println!("{}", i+1);
    }
}
