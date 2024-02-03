use std::process;
use std::thread;
use std::time::Duration;
use std::ptr;

fn main() {
    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Fork failed!");
            process::exit(1);
        }
        0 => {
            //Child process
            thread::sleep(Duration::from_secs(2));
            let result = unsafe { libc::wait(ptr::null_mut()) };
            println!("Result of wait in child was: {}", result);
        }
        child_pid => {
            let result = unsafe { libc::wait(ptr::null_mut()) };
            println!("Result of wait for child: {} was {}", child_pid, result);
        }
    }
}
