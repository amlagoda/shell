use libc::{close as c_close, pipe as c_pipe};
use libc::{grantpt as c_grantpt, posix_openpt as c_posix_openpt, O_NOCTTY, O_RDWR};
use libc::{open as c_open, ptsname as c_ptsname, unlockpt as c_unlockpt};
use std::ffi::CStr;
use std::io::Error;
use std::ops::Drop;

// last pipeline is pty, others - pipe
pub fn mass_create_pipes(count: usize) -> Result<Vec<Pipeline>, Error> {
    let mut pipelines: Vec<Pipeline> = vec![];

    for num in 0..count {
        let pipeline = Pipeline::try_new(if num == 0 {
            PipelineEndsType::Pty
        } else {
            PipelineEndsType::Pipe
        });

        if let Err(err) = pipeline {
            return Err(err);
        }

        pipelines.push(pipeline.unwrap());
    }

    pipelines.reverse();

    Ok(pipelines)
}

pub fn create_pipe() -> Result<Pipeline, Error> {
    Pipeline::try_new(PipelineEndsType::Pipe)
}

pub struct Pipeline {
    read_end: i32,
    write_end: i32,
}

impl Pipeline {
    pub fn try_new(ends_type: PipelineEndsType) -> Result<Pipeline, Error> {
        match ends_type {
            PipelineEndsType::Pipe => Pipeline::try_new_pipe(),
            PipelineEndsType::Pty => Pipeline::try_new_pty(),
        }
    }

    fn try_new_pipe() -> Result<Pipeline, Error> {
        let mut ends: [i32; 2] = [0; 2];
        let status = unsafe { c_pipe(ends.as_mut_ptr()) };

        if status == 0 {
            let pipeline = Pipeline {
                read_end: ends[0],
                write_end: ends[1],
            };

            Ok(pipeline)
        } else {
            Err(Error::other("pipe error"))
        }
    }

    fn try_new_pty() -> Result<Pipeline, Error> {
        let read_end = unsafe { c_posix_openpt(O_RDWR | O_NOCTTY) };
        if read_end == -1 {
            return Err(Error::other("posix_openpt error"));
        }

        if unsafe { c_grantpt(read_end) == -1 } {
            return Err(Error::other("grantpt error"));
        }

        if unsafe { c_unlockpt(read_end) == -1 } {
            return Err(Error::other("unlockpt error"));
        }

        let name_ptr = unsafe { c_ptsname(read_end) };
        if name_ptr.is_null() {
            return Err(Error::other("ptsname error"));
        }

        let name = unsafe { CStr::from_ptr(name_ptr) }
            .to_str()
            .map_err(|_| Error::other("from_ptr error"))?;

        let write_end = unsafe { c_open(name.as_ptr() as *const _, O_RDWR) };
        if write_end == -1 {
            return Err(Error::other("open error"));
        }

        let pipeline = Pipeline {
            read_end: read_end,
            write_end: write_end,
        };

        Ok(pipeline)
    }

    pub fn read_end(&self) -> i32 {
        self.read_end
    }

    pub fn write_end(&self) -> i32 {
        self.write_end
    }

    fn close(&mut self) {
        if self.read_end >= 0 {
            unsafe { c_close(self.read_end) };
            self.read_end = -1;
        }

        if self.write_end >= 0 {
            unsafe { c_close(self.write_end) };
            self.write_end = -1;
        }
    }

    pub fn close_write_end(&mut self) {
        if self.write_end >= 0 {
            unsafe { c_close(self.write_end) };
            self.write_end = -1;
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        self.close();
    }
}

pub enum PipelineEndsType {
    Pipe,
    Pty,
}
