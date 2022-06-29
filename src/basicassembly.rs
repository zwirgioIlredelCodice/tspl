use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tsplcore::{Tsvm, pcnext};

/// a prettier assebler with some helps
pub fn assembler(filename: &str, vm: &mut Tsvm) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_pc: String = String::from("");

    for line in reader.lines() {
        let line: String = line.unwrap();

        if line.starts_with("//") { // a comment do nothing
            continue;
        }

        let v: Vec<&str> = line.splitn(2, "__").collect();
        let key: String = String::from(v[0]);
        let value: String = String::from(v[1]);
        
        if key == "..." {
            if current_pc.is_empty() {
                panic!("... is permitted if only after a defiend program counter es. main^0 _\n...");
            }
            else {
                
                pcnext(&mut current_pc);
            }
        } else {
            current_pc = key.clone();
        }


        vm.mem.insert(current_pc.clone(), value);
    }
}
