use std::cmp::PartialEq;
use std::io::Error;

pub struct Loader {
    file_path: String,
    operation: Operation,
}

impl Loader {
    pub fn try_new(file_path: String, flag: &str) -> Result<Loader, Error> {
        let loader = Loader {
            file_path,
            operation: Operation::try_from(flag)?,
        };

        Ok(loader)
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn is_download(&self) -> bool {
        self.operation == Operation::Download
    }

    pub fn is_upload_append(&self) -> bool {
        self.operation == Operation::Upload(UploadMode::Append)
    }
}

enum Operation {
    Download,
    Upload(UploadMode),
}

impl PartialEq for Operation {
    fn eq(&self, other: &Operation) -> bool {
        match (self, other) {
            (Operation::Download, Operation::Download) => true,
            (Operation::Upload(a), Operation::Upload(b)) => a == b,
            _ => false,
        }
    }
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

impl PartialEq for UploadMode {
    fn eq(&self, other: &UploadMode) -> bool {
        match (self, other) {
            (UploadMode::Rewrite, UploadMode::Rewrite) => true,
            (UploadMode::Append, UploadMode::Append) => true,
            _ => false,
        }
    }
}
