use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


pub struct ChunkedLines {
    reader: BufReader<File>,
    chunk_size: usize,
}

impl ChunkedLines {
    pub fn new<P: AsRef<Path>>(path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self { reader, chunk_size })
    }
}

impl Iterator for ChunkedLines {
    type Item = io::Result<Vec<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => break, // End of file
                Ok(_) => {
                    chunk.push(line.trim()
                        .to_lowercase()
                        .replace('\n', ""));
                }
                Err(e) => return Some(Err(e)),
            }
        }
        if chunk.is_empty() {
            None
        } else {
            Some(Ok(chunk))
        }
    }
}