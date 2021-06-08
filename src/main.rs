use std::io;
use std::convert::TryFrom;

fn main() {
    let mut mul_val:i32 = 1;
    for _ in 0..3{
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("Failed");
        mul_val = mul_val * input_str.trim().parse::<i32>().expect("Failed");
    }

    let mut digit_list : Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    while mul_val != 0 {
        digit_list[usize::try_from(mul_val % 10).expect("Failed")] += 1;
        mul_val = (mul_val - (mul_val % 10)) / 10;
    }

    for item in digit_list.iter() {
        println!("{}", item);
    }
}
