use std::io::{BufRead, BufReader, Read};
use std::iter::Iterator;
use std::num::ParseFloatError;

use crate::error::*;
use crate::types::*;

pub struct Reader<T, R> {
    reader: BufReader<R>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, R> Iterator for Reader<T, R>
where
    T: std::str::FromStr<Err = ParseFloatError>,
    R: Read,
{
    type Item = Frame<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_snapshot().ok()
    }
}

macro_rules! parse_line {
    ($reader:expr) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer
    }};
    ($reader:expr, $t:ty) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer.trim().parse::<$t>()?
    }};
}

impl<T, R: Read> Reader<T, R> {
    pub fn new(inner: R) -> Self {
        Reader {
            reader: BufReader::new(inner),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn read_snapshot(&mut self) -> Result<Frame<T>>
    where
        T: std::str::FromStr<Err = std::num::ParseFloatError>,
    {
        let num_atoms = parse_line!(self.reader, i32);
        let comment = parse_line!(self.reader);

        let mut atoms: Vec<Atom<T>> = Vec::new();
        for _ in 0..num_atoms {
            atoms.push(parse_line!(self.reader, Atom<T>));
        }

        Ok(Frame {
            comment: comment,
            atoms: atoms,
        })
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
        let mut reader: Reader<f64, _> = Reader::new(data);
        let success = reader.read_snapshot();
        assert!(success.is_ok());

        let frame = success.unwrap();
        assert_eq!(3, frame.size());
    }

    #[test]
    fn test_iterator() {
        let data: &[u8] = b"\
            3
            1st frame
            C 1.0 2.0 3.0
            O 4.0 3.0 6.0
            H 5.0 1.5 4.0
            3
            2nd frame
            C 1.1 1.9 2.8
            O 4.2 3.0 5.9
            H 5.0 1.6 4.0";
        let mut reader: Reader<f64, _> = Reader::new(data);
        assert!(reader.next().is_some());
        assert!(reader.next().is_some());
        assert!(reader.next().is_none());
    }
}
