use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tsplcore::Tsvm;

pub fn assemblyfromfile(filename: &str, vm: &mut Tsvm, debug: bool) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let v: Vec<&str> = line.splitn(2, "__").collect();

        let key: String = String::from(v[0]);
        let value: String = String::from(v[1]);

        if debug {
            println!("{}. {}", index + 1, line);
            println!("key: {}, value: {}", key, value);
        }

        vm.mem.insert(key, value);
    }
}
