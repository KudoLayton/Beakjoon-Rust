use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let val = input.trim().parse::<i32>().expect("Failed");
    match val % 400 {
        0 => println!("1"),
        _ => {
            match val % 100 {
                0 => println!("0"),
                _ => {
                    match val % 4 {
                        0 => println!("1"),
                        _ => println!("0"),
                    }
                },
            }
        },
    }
}
