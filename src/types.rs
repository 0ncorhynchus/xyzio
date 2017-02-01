use std::str::FromStr;
use error::*;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Coordinate {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Coordinate{ x: x, y: y, z: z }
    }
}

#[derive(Debug, PartialEq)]
pub struct Atom {
    pub element: String,
    pub coordinate: Coordinate
}

impl Atom {
    fn new(element: &str, x: f32, y: f32, z: f32) -> Self {
        Atom {
            element: element.to_string(),
            coordinate: Coordinate::new(x,y,z)
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
                     splitted[1].parse::<f32>()?,
                     splitted[2].parse::<f32>()?,
                     splitted[3].parse::<f32>()?))
    }
}

pub struct Snapshot {
    pub comment: String,
    pub atoms: Vec<Atom>
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
}

