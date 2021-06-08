use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed");
    let v = a.split_whitespace();
    let r = v.clone().count();
    match r {
        2 => print!("{}", v.fold(0, |acc:i32, x| acc + x.parse::<i32>().expect("Nope"))),
        _ => (),
    }
}