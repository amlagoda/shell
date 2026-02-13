use libc::{close as c_close, pipe as c_pipe};
use std::io::Error;

fn mass_create(count: i32) -> Result<Vec<Pipeline>, Error> {
    let mut pipelines: Vec<Pipeline> = vec![];

    for _ in 0..count {
        let pipeline = Pipeline::try_new();

        if let Err(err) = pipeline {
            mass_close(pipelines);

            return Err(err);
        }

        pipelines.push(pipeline.unwrap());
    }

    Ok(pipelines)
}

fn mass_close(pipelines: Vec<Pipeline>) {
    for mut pipeline in pipelines {
        pipeline.close();
    }
}

struct Pipeline {
    read_end: u32,
    write_end: u32,
}

impl Pipeline {
    fn try_new() -> Result<Pipeline, Error> {
        let mut ends: [i32; 2] = [0; 2];
        let pipeline = Pipeline {
            read_end: ends[0] as u32,
            write_end: ends[1] as u32,
        };

        let status = unsafe { c_pipe(ends.as_mut_ptr()) };

        if status == 0 {
            Ok(pipeline)
        } else {
            Err(Error::other("pipeline creation error"))
        }
    }

    fn read_end(&self) -> u32 {
        self.read_end
    }

    fn write_end(&self) -> u32 {
        self.write_end
    }

    fn close(&mut self) {
        if self.read_end > 0 {
            unsafe { c_close(self.read_end as i32) };
            self.read_end = 0;
        }

        if self.write_end > 0 {
            unsafe { c_close(self.write_end as i32) };
            self.write_end = 0;
        }
    }

    fn close_write_end(&mut self) {
        if self.write_end > 0 {
            unsafe { c_close(self.write_end as i32) };
            self.write_end = 0;
        }
    }
}
