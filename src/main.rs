use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let case_num = input.trim().parse::<i32>().expect("Failed");
    for _ in 0..case_num{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let input_num : Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
        println!("{}", input_num[0] + input_num[1]);
    }
}
