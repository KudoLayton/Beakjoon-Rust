use std::io;

fn main() {
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Failed");
  let v = a.split_whitespace();
  let r = v.clone().count();
  match r {
    2 => {
      let m:Vec<f64>  = v.map(|x| x.parse::<f64>().expect("Not working")).collect();
      print!("{:.9}", m[0] / m[1]);
    }
    _ => (),
  }
}