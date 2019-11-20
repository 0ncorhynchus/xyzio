use std::io;

use crate::error::*;
use crate::types::*;
use std::fmt;

pub struct Writer<R> {
    buffer: R,
}

impl<R: io::Write> Writer<R> {
    pub fn new(inner: R) -> Self {
        Writer { buffer: inner }
    }

    pub fn write_snapshot<T>(&mut self, frame: &Frame<T>) -> Result<()>
    where
        T: fmt::Display,
    {
        writeln!(self.buffer, "{}", frame)?;
        self.buffer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = Writer::new(&mut buffer);
            let result = writer.write_snapshot(&Frame::new(
                "comment".to_string(),
                vec![
                    Atom::new("C", [10.2, 13.4, 8.9]),
                    Atom::new("N", [3.1, 10.8, 13.6]),
                    Atom::new("H", [5.7, 13.4, 4.6]),
                ],
            ));
            assert!(result.is_ok());
        }
        let string = String::from_utf8(buffer);
        assert!(string.is_ok());
        assert_eq!(
            "3\n\
             comment\n\
             C 10.2 13.4 8.9\n\
             N 3.1 10.8 13.6\n\
             H 5.7 13.4 4.6\n",
            string.unwrap()
        );
    }
}
