use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_num = input.trim().parse::<usize>().expect("Failed");
    for i in 1..(input_num + 1){
        println!("{}", "*".repeat(i));
    }
}
