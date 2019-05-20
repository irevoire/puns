use rand::prelude::*;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::str;

#[derive(Debug)]
pub struct Pun {
    file: File,
    size: u64,
    //    rng: ThreadRng,
    puns: Vec<String>,
}

impl Pun {
    pub fn new(filename: &str) -> Result<Self, &str> {
        let metadata = fs::metadata(filename).unwrap();
        let file = File::open(filename).unwrap();
        //        let rng = rand::thread_rng();

        Ok(Pun {
            file,
            size: metadata.len(),
            //           rng,
            puns: Vec::new(),
        })
    }

    fn fullfill_puns(&mut self) -> Result<(), &str> {
        let mut rng = rand::thread_rng(); // TODO: do this only once in thea new
        let mut buf = [0 as u8; 1024];
        let position = rng.gen_range(0, self.size);
        self.file.seek(SeekFrom::Start(position)).unwrap();
        self.file.read(&mut buf).unwrap();
        let pun = str::from_utf8(&buf).unwrap();

        self.puns = pun
            .split("\r\n")
            .skip(1) // remove the first pun because it's probably fucked up
            .filter_map(|s| s.splitn(2, ",").nth(1)) // remove the number before the pun
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        self.puns.pop();

        Ok(())
    }

    pub fn get(&mut self) -> String {
        loop {
            if let Some(pun) = self.puns.pop() {
                return pun;
            }
            self.fullfill_puns();
        }
    }
}
