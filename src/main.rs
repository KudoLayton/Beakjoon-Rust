use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let header: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_list = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed"));
    for num in input_list{
        if num < header[1] {
            print!("{} ", num);
        }
    }
}
