extern crate time;
use std::time::duration::Duration;

fn main() {
    let now = time::now_utc();
    let time_since_first = Duration::days((now.tm_mday - 1) as i64);
    let seconds_of_first_day = now.to_timespec() - time_since_first;
    let first_of_the_month = time::at_utc(seconds_of_first_day);

    print!("{:>11}", now.strftime("%B").ok().unwrap());
    println!(" {:<8}", now.strftime("%Y").ok().unwrap());
    println!("Su Mo Tu We Th Fr Sa");

    let one_day = Duration::hours(24);

    let this_month = now.tm_mon;
    let mut current_day = first_of_the_month;

    if current_day.tm_wday > 0 {
        let mut count = 0u32;
        while (count as i32) < current_day.tm_wday {
            print!("   ");
            count += 1;
        }
    }
    while current_day.tm_mon == this_month {
        print!("{:>2} ", current_day.tm_mday);
        if current_day.tm_wday == 6 {
            println!(""); // newline
        }
        current_day = current_day + one_day;
    }
    println!("");
}
