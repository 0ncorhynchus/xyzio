use std::io;
use std::num;
use std::str::FromStr;

#[derive(Debug)]
pub enum XYZError {
    IO(io::Error),
    IllegalState(String),
    Parse(num::ParseFloatError),
}

impl From<io::Error> for XYZError {
    fn from(err: io::Error) -> XYZError {
        XYZError::IO(err)
    }
}

impl From<num::ParseFloatError> for XYZError {
    fn from(err: num::ParseFloatError) -> XYZError {
        XYZError::Parse(err)
    }
}

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
    pub serial: String,
    pub coordinate: Coordinate
}

impl Atom {
    fn new(serial: &str, coordinate: Coordinate) -> Self {
        Atom{ serial: String::from(serial), coordinate: coordinate }
    }
}

impl FromStr for Atom {
    type Err = XYZError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split_whitespace().collect();
        if splitted.len() != 4 {
            return Err(XYZError::IllegalState(String::from("")));
        }
        let coordinate = try!(parse_coord(splitted[1], splitted[2], splitted[3]));
        Ok(Atom::new(splitted[0], coordinate))
    }
}

pub type Snapshot = (String, Vec<Atom>);

fn parse_coord(x_str: &str, y_str: &str, z_str: &str) -> Result<Coordinate, XYZError> {
    let x = try!(x_str.parse::<f32>());
    let y = try!(y_str.parse::<f32>());
    let z = try!(z_str.parse::<f32>());

    Ok(Coordinate::new(x,y,z))
}

pub fn read_xyz_line<R: io::BufRead>(reader: &mut R) -> Result<Atom, XYZError> {
    let mut buffer = String::new();
    try!(reader.read_line(&mut buffer));
    buffer.parse::<Atom>()
}

// fn collect_xyz<R: io::BufRead>(reader: &mut R, num: i32) -> Result<Vec<Atom>, XYZError> {
//     let mut vec = Vec::new();
//     for _ in 0..num {
//         match read_xyz_line(reader) {
//             Some(x) => vec.push(x),
//             _ => ()
//         };
//     }
//     vec
// }

// pub fn read_snapshot<R: io::BufRead>(mut reader: &mut R) -> Snapshot {
//     let mut buffer = String::new();
//     try!(reader.read_line(&mut buffer));
//     let num = i32::from_str(&buffer).unwrap(); // TODO try
//     reader.read_line(&mut buffer); // TODO try
//     (buffer.clone(), collect_xyz(reader, num))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let success = "C 10.0 11.0 12.0".parse::<Atom>();
        assert!(success.is_ok());
        assert_eq!(
            Atom::new("C", Coordinate::new(10.0, 11.0, 12.0)),
            success.unwrap());

        let failure = "C 1.0 2.0 a".parse::<Atom>();
        assert!(failure.is_err());
    }
}
