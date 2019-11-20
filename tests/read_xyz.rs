extern crate xyzio;

use std::fs::File;
use xyzio::{Atom, Reader};

#[test]
fn test_read_ferric_oxide() {
    let input = File::open("xyz/ferric_oxide.xyz").unwrap();
    let mut reader = Reader::new(input);
    let result = reader.read_snapshot();
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    assert_eq!(5, snapshot.size());
    assert_eq!(
        Atom::new("Fe", [-0.7145, 0.4125, 0.0000]),
        snapshot.atoms[0]
    );
    assert_eq!(Atom::new("Fe", [0.7145, 0.4125, 0.0000]), snapshot.atoms[1]);
    assert_eq!(Atom::new("O", [0.0000, 0.0000, 0.0000]), snapshot.atoms[2]);
    assert_eq!(Atom::new("O", [1.4289, 0.0000, 0.0000]), snapshot.atoms[3]);
    assert_eq!(Atom::new("O", [-1.4290, 0.0000, 0.0000]), snapshot.atoms[4]);
}
