pub mod pipes;

use crate::pipes::{close_pipes, create_pipes};
use libc::{fork, gettimeofday, sched_setaffinity, timeval};
use std::process::exit;

pub fn main() {
    //We need to create two processes and create a pipe between them to measure the context switch time.

    let pipes = match create_pipes() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    //Now use fork to create two child processes. They must be running on the same CPU
    // which we can make sure of by calling sched_setaffinity() on the parent process.

    unsafe {
        let this_process = libc::getpid();
        sched_setaffinity(this_process, 1, &1);
    }

    let pid = unsafe { fork() };

    if pid < 0 {
        eprintln!("fork failed");
        exit(1);
    }

    //At the end close the pipes
    close_pipes(&pipes);
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
