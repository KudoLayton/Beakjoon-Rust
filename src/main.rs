use std::io;
use std::collections::HashMap;

fn main() {
    let mut number_log: HashMap<i32, bool> = HashMap::new();
    for _ in 0..10{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("False");
        number_log.insert(input.trim().parse::<i32>().expect("Failed") % 42, true);
    }
    println!("{}", number_log.len());
}
