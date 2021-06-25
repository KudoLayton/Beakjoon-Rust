use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let count = input.trim().parse::<u32>().expect("Failed");

    let mut result = 665u32;
    for _ in 0..count {
        let mut max_count_six = 0u32;
        while max_count_six < 3 {
            result += 1;
            max_count_six = 0u32;
            let mut count_six = 0u32;
            let mut eval_result = result.clone();
            while eval_result != 0 {
                let remainer = eval_result % 10;
                match remainer {
                    6 => count_six += 1,
                    _ => {
                        max_count_six = max_count_six.max(count_six);
                        count_six = 0;
                    }
                }
                eval_result = (eval_result - remainer) / 10;
            }
            max_count_six = max_count_six.max(count_six);
        }
    }
    println!("{}", result);
}
