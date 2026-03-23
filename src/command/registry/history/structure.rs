use std::io::Error;

pub struct Loader {
    file_path: String,
    operation: Operation,
}

impl Loader {
    pub fn try_new(file_path: String, flag: &str) -> Result<Loader, Error> {
        let loader = Loader {
            file_path: file_path,
            operation: Operation::try_from(flag)?,
        };

        Ok(loader)
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn is_download(&self) -> bool {
        match self.operation {
            Operation::Download => true,
            _ => false,
        }
    }

    pub fn is_upload_append(&self) -> bool {
        match self.operation {
            Operation::Upload(UploadMode::Append) => true,
            _ => false,
        }
    }
}

enum Operation {
    Download,
    Upload(UploadMode),
}

impl Operation {
    fn try_from(flag: &str) -> Result<Operation, Error> {
        match flag {
            "-r" => Ok(Operation::Download),
            "-w" => Ok(Operation::Upload(UploadMode::Rewrite)),
            "-a" => Ok(Operation::Upload(UploadMode::Append)),
            _ => Err(Error::other("not supported")),
        }
    }
}

enum UploadMode {
    Rewrite,
    Append,
}
