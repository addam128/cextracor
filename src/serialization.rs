use std::{fs::{ File, OpenOptions }, str::from_utf8};
use std::path::{ PathBuf, Path };
use std::io::{ self, Write, Read };
use std::collections::HashMap;

use json::array;

use crate::utils;
use crate::analyzers::traits::Analyzer;



fn prompt(path: &Path) -> Result<bool, utils::Error> {
    loop {
        print!("The output file {:?} exists, do you want to overwrite its contents? [y/n]:", path);
        io::stdout().flush()?;
        let mut answer = [0u8; 1];
        io::stdin().read_exact(&mut answer)?;
        match from_utf8(&answer) {
            Ok(val) => {
                match  val {
                    "y" => { return Ok(true);}
                    "n" => { return Ok(false);}
                    _ => {println!("Invalid answer.")}
                }
            }
            Err(_) => {
                println!("Invalid char.");
            }
        }
    }
}




pub(crate) struct JsonSerializer {
    _outstream: File
}

impl JsonSerializer {

    pub(crate) fn new( 
        path: &str
    ) -> Result<Self, utils::Error>
{

        let mut _path = PathBuf::from(path);
        _path.set_extension("json");

        let ofile = match OpenOptions::new()
                                    .write(true)
                                    .create_new(true)
                                    .open(_path.as_path()) {
                                
                                Ok(handle) => {
                                    handle
                                }
                                Err(_) => {
                                    
                                    match prompt(_path.as_path())? {
                                        true => {
                                            OpenOptions::new()
                                                        .write(true)
                                                        .create(true)
                                                        .truncate(true)
                                                        .open(_path.as_path())?
                                        }
                                        false => {
                                            return Err(utils::Error::UserChoice);
                                        }

                                    }
                                }
        };

        Ok(
            Self {
            _outstream: ofile
            }
        )
    }

    pub(crate) fn serialize(
        &mut self,
        analyzers: &mut HashMap< String, Box<dyn Analyzer> >
    ) -> Result<(), utils::Error> {

        self._outstream.write_all(
            json::stringify_pretty(
                json::object!{
                    title: analyzers.get_mut("title").unwrap().finalize()?,
                    versions: analyzers.get_mut("versions").unwrap().finalize()?,
                    bibliography: analyzers.get_mut("bibliography").unwrap().finalize()?,
                    table_of_contents: array![],
                    revisions: analyzers.get_mut("revisions").unwrap().finalize()?
                },
                4)
                .as_bytes()
        )?;


        Ok(())

    } 

}