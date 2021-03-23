mod reader;
mod analyzers;
mod serialization;
mod models;
mod utils;

use std::env;
use std::process;

fn process(
    path: &str)
    -> Result<(), utils::Error> {

    reader::open_file(path)?;

    Ok(())
}

fn main() {

    let retval = 
        env::args()
        .map(
            move |arg: String| -> i32{
                match process(arg.as_str()) {
                    Ok(_) => { 0 }
                    Err(err) => {
                        match err {
                            utils::Error::IOError(err) => { eprintln!("Whoops: {}", err)}
                            utils::Error::Utf8ConversionError(err) => {eprintln!("Whoops: {}", err)}
                        }
                        1
                    }
                }
            }
        )
        .fold(0,
             |accumulator, elem| -> i32 { accumulator | elem });

    process::exit(retval);
}
