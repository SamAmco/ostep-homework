use libc::pipe;

pub struct pipes {
    pipe_1: [i32; 2],
    pipe_2: [i32; 2],
}

pub fn create_pipes() -> Result<pipes, &'static str> {
    let mut pipe_1_ids: [i32; 2] = [0, 0];
    let mut pipe_2_ids: [i32; 2] = [0, 0];

    let err = unsafe { pipe(pipe_1_ids.as_mut_ptr()) };

    if err != 0 {
        return Err("pipe failed");
    }

    let err2 = unsafe { pipe(pipe_2_ids.as_mut_ptr()) };

    if err2 != 0 {
        return Err("pipe failed");
    }

    Ok(pipes {
        pipe_1: pipe_1_ids,
        pipe_2: pipe_2_ids,
    })
}

// To close the pipes and clean up we call close() on the file descriptors.
pub fn close_pipes(p: &pipes) {
    unsafe {
        libc::close(p.pipe_1[0]);
        libc::close(p.pipe_1[1]);
        libc::close(p.pipe_2[0]);
        libc::close(p.pipe_2[1]);
    }
}
