use std::io;

fn main() {
    let mut input_count_str = String::new();
    io::stdin().read_line(&mut input_count_str).expect("Failed");
    let input_count = input_count_str.trim().parse::<i32>().expect("Failed") as f32;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_num = input.split_whitespace().map(|x| x.parse::<f32>().expect("Failed"));
    let input_max_val = input_num.clone().fold(0.0, |acc:f32, x| acc.max(x));
    let input_sum:f32 = input_num.sum();
    println!("{}", input_sum / (input_max_val * input_count) * 100.0);
}