use std::io;

fn main() {
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        match input.trim(){
            "0" => break,
            _ => {
                let x:i32 = input.trim().parse().expect("Failed");
                let rev_x:i32 = input.trim().chars().rev().collect::<String>().parse::<i32>().expect("Failed");
                if x == rev_x {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
        }
    }
}
