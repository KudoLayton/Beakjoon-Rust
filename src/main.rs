use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let iter_num = input.trim().parse::<usize>().expect("Failed");
    let mut dict:HashMap<String, bool> = HashMap::new();
    for _ in 0..iter_num{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        dict.insert(input, true);
    }
    let dict:Vec<_> = dict.keys().collect();
    let mut dict:Vec<_> = dict.iter().map(|x| x.trim()).collect();
    dict.sort_by(|x, y| 
        if x.len() > y.len(){
            return Ordering::Greater;
        } else if x.len() < y.len(){
            return Ordering::Less;
        }else{
            return x.cmp(y);
        }
    );
    for dict_string in dict{
        println!("{}", dict_string)
    }
}
