extern crate xyzio;

use std::fs::File;
use xyzio::{Atom, Snapshot, Writer};


#[test]
fn test_write_ferric_oxide() {
    let output = File::create("xyz/write_test.xyz").unwrap();
    let mut writer = Writer::new(output);
    let snapshot = Snapshot {
        comment: "ferric oxide".to_string(),
        atoms: vec![
            Atom::new("Fe", -0.7145, 0.4125, 0.0000),
            Atom::new("Fe",  0.7145, 0.4125, 0.0000),
            Atom::new("O",   0.0000, 0.0000, 0.0000),
            Atom::new("O",   1.4289, 0.0000, 0.0000),
            Atom::new("O",  -1.4290, 0.0000, 0.0000),
        ],
    };
    assert!(writer.write_snapshot(&snapshot).is_ok());
}
