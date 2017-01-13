// use std::io;
// use std::str::FromStr;
use std::num::ParseFloatError;

pub type Coordinate = (f32, f32, f32);
pub type XYZLine = (String, Coordinate);
pub type Snapshot = (String, Vec<XYZLine>);

pub fn parse_coord(x: &str, y: &str, z: &str) -> Result<Coordinate, ParseFloatError> {
    let vec_x = try!(x.parse::<f32>());
    let vec_y = try!(y.parse::<f32>());
    let vec_z = try!(z.parse::<f32>());

    Ok((vec_x, vec_y, vec_z))
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
