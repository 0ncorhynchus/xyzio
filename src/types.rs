use std::str::FromStr;
use error::*;

type Real = f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub element: String,
    pub x: Real,
    pub y: Real,
    pub z: Real
}

impl Atom {
    fn new(element: &str, x: Real, y: Real, z: Real) -> Self {
        Atom {
            element: element.to_string(),
            x: x,
            y: y,
            z: z
        }
    }
}

impl FromStr for Atom {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let splitted: Vec<&str> = s.split_whitespace().collect();
        if splitted.len() != 4 {
            return Err(Error::IllegalState(String::from("")));
        }
        Ok(Atom::new(splitted[0],
                     splitted[1].parse::<Real>()?,
                     splitted[2].parse::<Real>()?,
                     splitted[3].parse::<Real>()?))
    }
}

pub struct Snapshot {
    pub comment: String,
    pub atoms: Vec<Atom>
}

impl Snapshot {
    pub fn size(&self) -> usize {
        self.atoms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let success = "C 10.0 11.0 12.0".parse::<Atom>();
        assert!(success.is_ok());
        assert_eq!(
            Atom::new("C", 10.0, 11.0, 12.0),
            success.unwrap());

        let failure = "C 1.0 2.0 a".parse::<Atom>();
        assert!(failure.is_err());
    }

    #[test]
    fn test_snapshot() {
        let snapshot = Snapshot {
            comment: "This is a comment".to_string(),
            atoms: vec![
                Atom::new("C", 10.0, 11.0, 12.0),
                Atom::new("O",  8.4, 12.8,  5.0),
                Atom::new("H", 23.0,  9.0, 11.8),
            ]
        };
        assert_eq!(3, snapshot.size());
        assert_eq!("This is a comment", snapshot.comment);
        assert_eq!(Atom::new("C", 10.0, 11.0, 12.0),
                   snapshot.atoms[0]);
        assert_eq!(Atom::new("O",  8.4, 12.8,  5.0),
                   snapshot.atoms[1]);
        assert_eq!(Atom::new("H", 23.0,  9.0, 11.8),
                   snapshot.atoms[2]);
    }
}

