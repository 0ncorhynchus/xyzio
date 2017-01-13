// use std::io;
// use std::str::FromStr;
use std::num;

pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Coordinate {
    fn new(x: f32, y: f32, z: f32) -> Coordinate {
        Coordinate{ x: x, y: y, z: z }
    }
}

pub type XYZLine = (String, Coordinate);
pub type Snapshot = (String, Vec<XYZLine>);

#[derive(Debug)]
pub enum XYZError {
    ParseError(num::ParseFloatError),
}

impl From<num::ParseFloatError> for XYZError {
    fn from(err: num::ParseFloatError) -> XYZError {
        XYZError::ParseError(err)
    }
}

pub fn parse_coord(x_str: &str, y_str: &str, z_str: &str) -> Result<Coordinate, XYZError> {
    let x = try!(x_str.parse::<f32>());
    let y = try!(y_str.parse::<f32>());
    let z = try!(z_str.parse::<f32>());

    Ok(Coordinate::new(x,y,z))
}

// fn read_xyz_line<R: io::BufRead>(reader: &mut R) -> Option<XYZLine> {
//     let mut buffer = String::new();
//     try!(reader.read_line(&mut buffer));
//     let splited: Vec<&str> = buffer.split_whitespace().collect();
//     if splited.len() == 4 {
//         Some(
//             (String::from(splited[0]),
//             parse_coord(splited[1], splited[2], splited[3]))
//             )
//     } else {
//         None
//     }
// }

// fn collect_xyz<R: io::BufRead>(reader: &mut R, num: i32) -> Vec<XYZLine> {
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
    #[test]
    fn it_works() {
    }
}
