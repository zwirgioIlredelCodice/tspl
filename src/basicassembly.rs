use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tsplcore::Tsvm;

pub fn assemblyfromfile(filename: &str,vm: &mut Tsvm) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // Show the line and its number.
        // println!("{}. {}", index + 1, line);
        
        let v: Vec<&str> = line.split("__").collect();
        if v.len() != 2 {
            panic!("token not recognised at line {}", index + 1);
        } else {
            vm.mem.insert(String::from(v[0]), String::from(v[1]));
        }
    }
}