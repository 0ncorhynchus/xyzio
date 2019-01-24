use std::io;

use error::*;
use types::*;

pub struct Writer<R> {
    buffer: R,
}

impl<R: io::Write> Writer<R> {
    pub fn new(inner: R) -> Self {
        Writer { buffer: inner }
    }

    pub fn write_snapshot(&mut self, snapshot: &Snapshot) -> Result<()> {
        self.buffer.write(snapshot.size().to_string().as_bytes())?;
        self.buffer.write(b"\n")?;
        self.buffer.write(snapshot.comment.as_bytes())?;
        self.buffer.write(b"\n")?;
        for atom in &snapshot.atoms {
            self.buffer.write(atom.to_string().as_bytes())?;
            self.buffer.write(b"\n")?;
        }
        self.buffer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;

    #[test]
    fn test_write() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = Writer::new(&mut buffer);
            let result = writer.write_snapshot(&Snapshot {
                comment: "comment".to_string(),
                atoms: vec![
                    Atom::new("C", 10.2, 13.4, 8.9),
                    Atom::new("N", 3.1, 10.8, 13.6),
                    Atom::new("H", 5.7, 13.4, 4.6),
                ],
            });
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
