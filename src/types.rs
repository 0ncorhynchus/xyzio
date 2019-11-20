use crate::error::*;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq, Clone)]
pub struct AtomBase<T> {
    pub element: String,
    pub position: T,
}

pub type Atom<T> = AtomBase<[T; 3]>;

impl<T> Atom<T> {
    pub fn new(element: &str, x: T, y: T, z: T) -> Self {
        Atom {
            element: element.to_string(),
            position: [x, y, z],
        }
    }
}

impl<T> FromStr for Atom<T>
where
    T: FromStr<Err = ParseFloatError>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let splitted: Vec<&str> = s.split_whitespace().collect();
        if splitted.len() != 4 {
            return Err(Error::IllegalState(String::from("")));
        }
        Ok(Atom::new(
            splitted[0],
            splitted[1].parse()?,
            splitted[2].parse()?,
            splitted[3].parse()?,
        ))
    }
}

impl<T: fmt::Display> fmt::Display for Atom<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.element, self.position[0], self.position[1], self.position[2]
        )
    }
}

pub struct Frame<T> {
    pub comment: String,
    pub atoms: Vec<Atom<T>>,
}

impl<T> Frame<T> {
    pub fn size(&self) -> usize {
        self.atoms.len()
    }
}

impl<T: fmt::Display> fmt::Display for Frame<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.size())?;
        write!(f, "{}", self.comment)?;
        for atom in &self.atoms {
            writeln!(f, "")?;
            write!(f, "{}", atom)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let success = "C 10.0 11.0 12.0".parse();
        assert!(success.is_ok());
        assert_eq!(Atom::new("C", 10.0, 11.0, 12.0), success.unwrap());

        let failure: Result<Atom<f64>> = "C 1.0 2.0 a".parse();
        assert!(failure.is_err());
    }

    #[test]
    fn test_atom_to_string() {
        let atom = Atom::new("C", 11.2, 8.5, 14.8);
        assert_eq!("C 11.2 8.5 14.8", atom.to_string());
    }

    #[test]
    fn test_snapshot() {
        let frame = Frame {
            comment: "This is a comment".to_string(),
            atoms: vec![
                Atom::new("C", 10.0, 11.0, 12.0),
                Atom::new("O", 8.4, 12.8, 5.0),
                Atom::new("H", 23.0, 9.0, 11.8),
            ],
        };
        assert_eq!(3, frame.size());
        assert_eq!("This is a comment", frame.comment);
        assert_eq!(Atom::new("C", 10.0, 11.0, 12.0), frame.atoms[0]);
        assert_eq!(Atom::new("O", 8.4, 12.8, 5.0), frame.atoms[1]);
        assert_eq!(Atom::new("H", 23.0, 9.0, 11.8), frame.atoms[2]);
    }

    #[test]
    fn test_format_snapshot() {
        let frame = Frame {
            comment: "This is a comment".to_string(),
            atoms: vec![
                Atom::new("C", 10.0, 11.0, 12.0),
                Atom::new("O", 8.4, 12.8, 5.0),
                Atom::new("H", 23.0, 9.0, 11.8),
            ],
        };
        assert_eq!(
            format!("{}", frame),
            "3\n\
             This is a comment\n\
             C 10 11 12\n\
             O 8.4 12.8 5\n\
             H 23 9 11.8"
        );
    }
}
