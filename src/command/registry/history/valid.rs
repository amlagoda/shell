use std::io::Error;

pub fn validate(args: Option<&Vec<&str>>) -> Result<Validated, Error> {
    const LOAD_FLAGS: [&str; 3] = ["-r", "-w", "-a"];
    const MODE_NOT_DEFINED: u8 = 0;
    const MODE_PRINT: u8 = 1;
    const MODE_LOAD: u8 = 2;
    let mut mode = MODE_NOT_DEFINED;

    let err = Error::other("incorrect arguments");
    let mut load_datasets: Option<Vec<LoadDataset>> = None;
    let mut count: Option<usize> = None;

    if args.is_none() {
        return Ok(Validated::from(count, load_datasets));
    }

    let mut iter = args.unwrap().iter();
    while let Some(arg) = iter.next() {
        if let Ok(parsed) = arg.parse::<usize>() {
            if mode != MODE_NOT_DEFINED {
                return Err(err);
            }

            mode = MODE_PRINT;
            count = Some(parsed);
        } else if LOAD_FLAGS.contains(arg) {
            if ![MODE_NOT_DEFINED, MODE_LOAD].contains(&mode) {
                return Err(err);
            }

            mode = MODE_LOAD;
            let file_path = iter.next();

            if file_path.is_none() {
                return Err(err);
            }

            let load_dataset = LoadDataset::new(file_path.unwrap().to_string(), arg.to_string());

            load_datasets = if let Some(mut items) = load_datasets {
                items.push(load_dataset);
                Some(items)
            } else {
                Some(vec![load_dataset])
            };
        } else {
            return Err(err);
        }
    }

    Ok(Validated::from(count, load_datasets))
}

pub struct Validated {
    count: Option<usize>,
    load_datasets: Option<Vec<LoadDataset>>,
}

impl Validated {
    fn from(count: Option<usize>, load_datasets: Option<Vec<LoadDataset>>) -> Validated {
        Validated {
            count,
            load_datasets,
        }
    }

    pub fn get_count(&self) -> Option<usize> {
        self.count
    }

    pub fn get_load_datasets(&self) -> Option<Vec<&LoadDataset>> {
        self.load_datasets.as_ref().map(|v| v.iter().collect())
    }
}

pub struct LoadDataset {
    path: String,
    flag: String,
}

impl LoadDataset {
    fn new(path: String, flag: String) -> LoadDataset {
        LoadDataset { path, flag }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn flag(&self) -> &str {
        self.flag.as_str()
    }
}
