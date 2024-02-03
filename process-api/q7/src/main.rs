use std::process;
use std::ptr;
use std::io;
use std::os::unix::io::AsRawFd;


fn main() {
    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Fork failed!");
            process::exit(1);
        }
        0 => {
            //Child process
            println!("Child can print");
            let fd = io::stdout().as_raw_fd();
            unsafe { libc::close(fd); }
            println!("Child can not print");
        }
        _ => {
            unsafe { libc::wait(ptr::null_mut()) };
            println!("Finished wating for child");
        }
    }
}
