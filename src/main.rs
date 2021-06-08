use std::io;
use std::collections::HashMap;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed");
  let input = input.trim().to_uppercase();
  let input_char = input.chars();
  let mut input_char_count_list: HashMap<char, i32> = HashMap::new();
  let mut max_count = 0;

  for c in input_char{
    let past_count = input_char_count_list.get(&c);
    match past_count{
      None => {
        input_char_count_list.insert(c, 1);
        max_count = max_count.max(1);
      },
      Some(_) => {
        let count = past_count.unwrap().clone() + 1;
        input_char_count_list.insert(c, count);
        max_count = max_count.max(count);
      },
    }
  }

  let mut is_max_count_repeated = false;
  let mut max_c = '\0';
  for (c, count) in input_char_count_list{
    match (max_count == count, is_max_count_repeated){
      (true, false) => {
        max_c = c;
        is_max_count_repeated = true;
      },
      (true, true) => {
        println!("?");
        return
      },
      (false, _) => (),
    }
  }
  println!("{}", max_c);
}
