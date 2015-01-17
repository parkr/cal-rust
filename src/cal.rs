extern crate time;
use time::*;

fn tm_to_string(time: Tm) -> &str {
  return "I AM A STRING";
}

fn main() {
  println!("This is my time: {}", tm_to_string(now()));
}
