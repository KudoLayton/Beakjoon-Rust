use std::io;

fn main() {
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        match input.trim(){
            "0 0" => break,
            _ => {
                let input_nums:Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
                println!("{}", input_nums[0] + input_nums[1]);
            }
        }
    }
}
