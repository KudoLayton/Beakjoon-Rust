use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let number_list= input.trim().chars().map(|x| x.to_string().parse::<i32>().expect("Failed"));
    println!("{}", number_list.fold(0, |acc, x| acc + x));
}
