use std::fs::{ OpenOptions, File };
use std::path::Path;
use std::io::Read;
use std::str::from_utf8;
use std::collections::HashMap;


use crate::utils;
use crate::analyzers::traits::Analyzer;

const CHUNK_SIZE: usize = 8192;
const BACKUP_LEN: usize = 12;

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

    let mut buffer:Vec<u8> = vec![0u8; CHUNK_SIZE];
    buffer.reserve(BACKUP_LEN);
   //let mut buffer: String = String::new();

    loop {
        buffer.truncate(CHUNK_SIZE);
        
        let mut bytes_read = input.read(buffer.as_mut())?;
        
        if bytes_read == 0 { break Ok(()); }
        let mut counter = 3;
        let chunk_str: &str = loop { // if an utf-8 char would get split, we need to handle this by trying to read some(1) bytes iteratively
                                    // because it is checked after each + byte this is kinda slow, but happens with probs x: lim x = 0
                                    // also the max iterations is at most 3 (max missing bytesfrom utf-8)
            if counter < 0 {
                return Err(utils::Error::BadReadError)
            }
             match from_utf8(buffer[..bytes_read].as_ref()) {
                Ok(slice) => { 
                     break slice;
                    }
                Err(_) => {
                    let mut byte = [0u8; 1];
                    input.read_exact(&mut byte)?;
                    buffer.push(byte[0]);
                    bytes_read += 1;
                }
            }
            counter -= 1;
        };
          
        for analyzer in analyzers.values_mut() {
            analyzer.process(chunk_str)?;
        }

    }
} 