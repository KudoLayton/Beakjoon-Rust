use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_chars = input.chars();
    for c in 'a'..='z'{
        let mut loop_input_chars = input_chars.clone();
        match loop_input_chars.position(|x| x == c){
            Some(x) => print!("{} ", x),
            None => print!("-1 "),
        }
    }
}
