use std::fs::{ OpenOptions, File };
use std::path::Path;
use std::io::Read;
use std::str::from_utf8;
use std::collections::HashMap;


use crate::utils;
use crate::analyzers::traits::Analyzer;

const CHUNK_SIZE: usize = 8192;

pub(crate) fn open_file<P>(
    path: P)
    -> Result<File, utils::Error> 
    where P: AsRef<Path>   
{

    Ok(
        OpenOptions::new()
                    .read(true)
                    .create(false)
                    .open(path)?
    )
}


pub(crate) fn read_and_process_chunks<I>(
    mut input: I,
    analyzers: &mut HashMap< String, Box<dyn Analyzer> >)
    -> Result<(), utils::Error>
    where I: Read,
{

    let mut buffer: [u8; CHUNK_SIZE] = [0u8; CHUNK_SIZE];

    loop {
        
        let bytes_read = input.read(&mut buffer)?;
        
        if bytes_read == 0 { break Ok(());}

        let chunk_str_slice = from_utf8(&mut buffer[..bytes_read])?;
         
        
        for analyzer in analyzers.values_mut() {
            analyzer.process(chunk_str_slice)?;
        }
    }
} 