//Just print the result of the linux sys-call gettimeofday()

use libc::{gettimeofday, timeval};
use std::process::exit;

pub fn main() {
    for _ in 0..100 {
        match get_time_of_day() {
            Ok(tv) => print_timeval(&tv),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

fn print_timeval(tv: &timeval) {
    println!("{}, {}", tv.tv_sec, tv.tv_usec);
}

fn get_time_of_day() -> Result<timeval, &'static str> {
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let err = unsafe { gettimeofday(&mut tv, std::ptr::null_mut()) };

    if err != 0 {
        return Err("gettimeofday failed");
    }

    Ok(tv)
}
