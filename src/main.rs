use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let test_case_num = input.trim().parse::<i32>().expect("Failed");

    for _ in 0..test_case_num{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let split_input_list:Vec<_> = input.split_whitespace().collect();
        let rep_num = split_input_list[0].parse::<i32>().expect("Failed");
        let char_list = split_input_list[1].chars();
        for item in char_list{
            for _ in 0..rep_num{
                print!("{}", item);
            }
        }
        print!("\n");
    }
}
