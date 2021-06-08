use std::io;
use std::io::Read;

fn main() {
    let mut input : Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).expect("Failed");
    let input_str:String = input.iter().map(|x| char::from(x.clone())).collect();
    let input_lines = input_str.as_str().trim().split("\n");
    for line in input_lines{
        let nums: Vec<_> = line.clone().split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
        println!("{}", nums[0] + nums[1]);
    }
}
