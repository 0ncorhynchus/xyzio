use std::io;
use std::io::prelude::BufRead;

use error::*;
use types::*;

pub struct Reader<R> {
    reader: io::BufReader<R>,
}

macro_rules! parse_line {
    ($reader:ident) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer
    }};
    ($reader:ident, $t:ty) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer.parse::<$t>()?
    }}
}

impl<R: io::Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Reader {
            reader: io::BufReader::new(inner)
        }
    }

    pub fn snapshot(&mut self) -> Result<Snapshot> {
        let reader = &mut self.reader;

        let num_atoms = parse_line!(reader, i32);
        let comment = parse_line!(reader);

        let mut atoms: Vec<Atom> = Vec::new();
        for _ in 0..num_atoms {
            atoms.push(parse_line!(reader, Atom));
        }

        Ok(Snapshot {
            comment: comment,
            atoms: atoms
        })
    }
}
