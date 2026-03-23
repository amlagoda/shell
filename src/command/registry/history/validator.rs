use crate::command::registry::history::structure::Loader;
use std::io::Error;

pub fn validate(args: Option<&Vec<&str>>) -> Result<(Option<usize>, Option<Vec<Loader>>), Error> {
    const LOAD_FLAGS: [&str; 3] = ["-r", "-w", "-a"];
    const MODE_NOT_DEFINED: u8 = 0;
    const MODE_PRINT: u8 = 1;
    const MODE_LOAD: u8 = 2;
    let mut mode = MODE_NOT_DEFINED;

    let err = Error::other("incorrect arguments");
    let mut loaders: Option<Vec<Loader>> = None;
    let mut count: Option<usize> = None;

    if args.is_none() {
        return Ok((count, loaders));
    }

    let mut iter = args.unwrap().into_iter();
    while let Some(arg) = iter.next() {
        let arg = *arg;

        if let Ok(parsed) = arg.parse::<usize>() {
            if mode != MODE_NOT_DEFINED {
                return Err(err);
            }

            mode = MODE_PRINT;
            count = Some(parsed);
        } else if LOAD_FLAGS.contains(&arg) {
            if ![MODE_NOT_DEFINED, MODE_LOAD].contains(&mode) {
                return Err(err);
            }

            mode = MODE_LOAD;
            let file_path = iter.next();

            if file_path.is_none() {
                return Err(err);
            }

            let loader = Loader::try_new(file_path.unwrap().to_string(), arg)?;

            loaders = if let Some(mut items) = loaders {
                items.push(loader);
                Some(items)
            } else {
                Some(vec![loader])
            };
        } else {
            return Err(err);
        }
    }

    Ok((count, loaders))
}
