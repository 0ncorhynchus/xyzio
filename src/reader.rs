use std::io;
use std::io::prelude::BufRead;
use std::iter::Iterator;

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
        buffer.trim().parse::<$t>()?
    }};
}

impl<R: io::Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Reader {
            reader: io::BufReader::new(inner),
        }
    }

    pub fn read_snapshot<T: std::str::FromStr<Err = std::num::ParseFloatError>>(
        &mut self,
    ) -> Result<Snapshot<T>> {
        let reader = &mut self.reader;

        let num_atoms = parse_line!(reader, i32);
        let comment = parse_line!(reader);

        let mut atoms: Vec<Atom<T>> = Vec::new();
        for _ in 0..num_atoms {
            atoms.push(parse_line!(reader, Atom<T>));
        }

        Ok(Snapshot {
            comment: comment,
            atoms: atoms,
        })
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = Snapshot<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_snapshot().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let data: &[u8] = b"\
            3
            comment
            C 1.0 2.0 3.0
            O 4.0 3.0 6.0
            H 5.0 1.5 4.0";
        let mut reader = Reader::new(data);
        let success = reader.read_snapshot();
        assert!(success.is_ok());

        let snapshot = success.unwrap();
        assert_eq!(3, snapshot.size());
    }

    #[test]
    fn test_iterator() {
        let data: &[u8] = b"\
            3
            1st snapshot
            C 1.0 2.0 3.0
            O 4.0 3.0 6.0
            H 5.0 1.5 4.0
            3
            2nd snapshot
            C 1.1 1.9 2.8
            O 4.2 3.0 5.9
            H 5.0 1.6 4.0";
        let mut reader = Reader::new(data);
        assert!(reader.next().is_some());
        assert!(reader.next().is_some());
        assert!(reader.next().is_none());
    }
}
