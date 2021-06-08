use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let case_num = input.trim().parse::<i32>().expect("Failed"); 
    for _ in 0..case_num{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        println!("{}", input.trim().split("X").fold(0, |acc, x| {
            let length = x.chars().count();
            acc + (length * (length + 1) / 2)
        }))
    }
}
