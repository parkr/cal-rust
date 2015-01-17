extern crate time;
use time::*;

fn main() {
    let time = now();
    println!("This is my time: {}", time.rfc3339());
}
