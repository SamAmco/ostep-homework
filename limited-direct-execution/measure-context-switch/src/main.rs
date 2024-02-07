//Just print the result of the linux sys-call gettimeofday()

use libc::{gettimeofday, timeval};

fn main() {
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let time = unsafe { gettimeofday(&mut tv, std::ptr::null_mut()) };

    println!("{}", time);
}
