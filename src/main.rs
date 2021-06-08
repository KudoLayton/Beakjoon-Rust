use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let time: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Failed")).collect();
    match (time[0] > 0, time[1] >= 45){
        (_, true) => println!("{} {}", time[0], time[1] - 45),
        (true, false) => println!("{} {}", time[0] - 1, time[1] + 15),
        (false, false) => println!("23 {}", time[1] + 15),
    }
}
