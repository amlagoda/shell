use libc::{close as c_close, pipe as c_pipe};
use std::io::Error;

struct Pipeline {
    read_end: u32,
    write_end: u32,
}

impl Pipeline {
    fn try_new() -> Result<Pipeline, Error> {
        let mut ends: [i32; 2] = [0; 2];
        let status = unsafe { c_pipe(ends.as_mut_ptr()) };

        if status == 0 {
            Ok(Pipeline {
                read_end: ends[0] as u32,
                write_end: ends[1] as u32,
            })
        } else {
            Err(Error::other("pipe error"))
        }
    }

    fn get_read_end(&self) -> u32 {
        self.read_end
    }

    fn get_write_end(&self) -> u32 {
        self.write_end
    }

    fn close(&mut self) -> () {
        if self.read_end > 0 {
            unsafe {
                c_close(self.read_end as i32);
            };
            self.read_end = 0;
        }

        if self.write_end > 0 {
            unsafe {
                c_close(self.write_end as i32);
            };
            self.write_end = 0;
        }
    }

    fn close_write_end(&mut self) -> () {
        if self.write_end > 0 {
            unsafe {
                c_close(self.write_end as i32);
            };
            self.write_end = 0;
        }
    }
}
