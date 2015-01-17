extern crate time;
use time::*;

fn tm_to_string(time: Tm) -> Str {
  return format!("BBB {}", time);
}

fn main() {
  println!("This is my time: {}", tm_to_string(now()));
}
