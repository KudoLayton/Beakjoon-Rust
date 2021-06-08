use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let rep = input.trim().parse::<i32>().expect("Failed");

    let mut output = String::new();
    for i in 0..rep{
        output.push_str(format!("{}\n", i+1).as_str());
    }
    print!("{}", output);
}
