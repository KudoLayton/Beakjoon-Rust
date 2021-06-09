use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_params: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
    let hor_min = input_params[0].min(input_params[2] - input_params[0]);
    let ver_min = input_params[1].min(input_params[3] - input_params[1]);
    println!("{}", hor_min.min(ver_min));
}
