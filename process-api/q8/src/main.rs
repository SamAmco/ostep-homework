use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::process;
use std::ptr;

fn main() {
    let mut pipe_ids: [c_int; 2] = [0; 2];
    if (unsafe { libc::pipe(pipe_ids.as_mut_ptr()) } != 0) {
        eprintln!("Pipe failed!");
        process::exit(1);
    }

    let first_child = unsafe { libc::fork() };

    match first_child {
        -1 => {
            eprintln!("Fork failed!");
            process::exit(1);
        }
        0 => {
            //First child process
            unsafe {
                libc::close(pipe_ids[0]);
                libc::dup2(pipe_ids[1], libc::STDOUT_FILENO);
                libc::close(pipe_ids[1]);

                execute_command("ls");
            }

            //We should never get here as execvp takes over
            eprintln!("execvp failed!");
            process::exit(1);
        }
        _ => {
            //Parent process
            let second_child = unsafe { libc::fork() };

            match second_child {
                -1 => {
                    eprintln!("Second fork failed!");
                }
                0 => {
                    //Child 2 process
                    unsafe {
                        libc::close(pipe_ids[1]);
                        libc::dup2(pipe_ids[0], libc::STDIN_FILENO);
                        libc::close(pipe_ids[0]);

                        execute_command("grep toml");
                    }

                    //We should never get here as execvp takes over
                    eprintln!("execvp failed!");
                    process::exit(1);
                }
                _ => {
                    //Parent process
                    unsafe {
                        libc::close(pipe_ids[0]);
                        libc::close(pipe_ids[1]);

                        libc::waitpid(first_child, ptr::null_mut(), 0);
                        libc::waitpid(second_child, ptr::null_mut(), 0);
                    }
                }
            }
        }
    }
}

fn execute_command(command: &str) {
    let args: Vec<CString> = command
        .split_whitespace()
        .map(|arg| CString::new(arg).expect("CString::new failed"))
        .collect();

    let args_ptrs: Vec<*const c_char> = args
        .iter()
        .map(|s| s.as_ptr())
        .chain(Some(ptr::null()))
        .collect();

    unsafe {
        libc::execvp(args[0].as_ptr(), args_ptrs.as_ptr());

        //We should never get here as execvp takes over
        eprintln!("execvp failed!");
        process::exit(1);
    }
}
