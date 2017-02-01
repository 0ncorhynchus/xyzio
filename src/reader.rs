use std::io;
use std::io::prelude::BufRead;

use error::*;
use types::*;

pub struct Reader<R> {
    reader: io::BufReader<R>,
}

impl<R: io::Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Reader {
            reader: io::BufReader::new(inner)
        }
    }

    pub fn snapshot(&mut self) -> Result<Snapshot> {
        let reader = &mut self.reader;
        let mut buffer = String::new();

        reader.read_line(&mut buffer)?;
        let num_atoms = buffer.parse::<i32>()?;

        let mut comment = String::new();
        reader.read_line(&mut comment)?;

        let mut atoms: Vec<Atom> = Vec::new();
        for _ in 0..num_atoms {
            reader.read_line(&mut buffer)?;
            atoms.push(buffer.parse::<Atom>()?);
        }

        Ok(Snapshot {
            comment: comment,
            atoms: atoms
        })
    }
}
