use std::collections::HashMap;

pub struct Tsvm {
    pub isrunning: bool,
    pub acc: String,
    pub pc: String,
    pub mem: HashMap<String, String>,
}

pub fn tsvminit() -> Tsvm {
    Tsvm {
        isrunning: false,
        acc: String::from(""),
        pc: String::from(""),
        mem: HashMap::new(),
    }
}

fn pcnext(pc: &mut String) {
    let v: Vec<&str> = pc.splitn(2, '^').collect();
    let mut n: i32 = v[1].parse().unwrap();
    n += 1;
    let name: String = String::from(v[0]);
    let num: String = n.to_string();
    pc.clear();
    pc.push_str(&name);
    pc.push('^');
    pc.push_str(&num);
}

fn exec(vm: &mut Tsvm, debug: bool) {

    if debug {
        println!("DB pc: {}", vm.pc);
    }
    let instruction: String = vm.mem.get(&vm.pc).expect("not found entry for pc").clone();

    //parse instruction
    let instructionlist: Vec<&str> = instruction.splitn(2, '^').collect();
    let command: &str = instructionlist[0];

    if debug {
        println!("DB command {:?}", instructionlist);
    }
    // memory
    if command == "load" {
        vm.acc = String::from(instructionlist[1]);
        pcnext(&mut vm.pc);
    } else if command == "get" {
        vm.acc = String::from(&vm.mem[instructionlist[1]]);
        pcnext(&mut vm.pc);
    } else if command == "set" {
        vm.mem
            .insert(String::from(instructionlist[1]), vm.acc.clone());
            pcnext(&mut vm.pc);
    } else if command == "del" {
        vm.mem.remove(&vm.acc);
        pcnext(&mut vm.pc);
    }
    // logic
    else if command == "add" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = vm.mem.get(instructionlist[1])
                            .expect("entry not found")
                            .parse()
                            .expect("not a number");
        let n3: i32 = n1 + n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "sub" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = vm.mem.get(instructionlist[1])
                            .expect("entry not found")
                            .parse()
                            .expect("not a number");
        let n3: i32 = n1 - n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "and" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = vm.mem.get(instructionlist[1])
                            .expect("entry not found")
                            .parse()
                            .expect("not a number");
        let n3: i32 = n1 & n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "or" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = vm.mem.get(instructionlist[1])
                            .expect("entry not found")
                            .parse()
                            .expect("not a number");();
        let n3: i32 = n1 | n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "not" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = !n1;
        vm.acc = n2.to_string();
        pcnext(&mut vm.pc);
    }
    // I/O
    else if command == "input" {
        std::io::stdin().read_line(&mut vm.acc).unwrap();
        pcnext(&mut vm.pc);
    } else if command == "output" {
        print!("{}", vm.acc);
        pcnext(&mut vm.pc);
    }
    // jumps
    else if command == "jump" {
        vm.pc = vm.acc.clone();
    } else if command == "jump0" {
        if vm.acc == "0" {
            vm.pc = String::from(instructionlist[1]);
        }
    }
    // default
    else if command == "stop" {
        vm.isrunning = false;
    } else {
        vm.isrunning = false;
    }
}

pub fn execmain(mut vm: Tsvm, debug: bool) -> Tsvm {
    vm.isrunning = true;
    vm.pc = String::from(&vm.mem["start"]);

    while vm.isrunning {
        exec(&mut vm, debug);
    }

    vm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcnext() {
        let mut s: String = String::from("main^0");
        pcnext(&mut s);
        assert_eq!(String::from("main^1"), s)
    }
    #[test]
    #[should_panic]
    fn test_notpcnext() {
        _ = pcnext(&mut String::from("main0"));
    }
    #[test]
    fn test_tsvminit() {
        let vm: Tsvm = tsvminit();
        assert_eq!(vm.isrunning,  false);
        assert_eq!(vm.acc, ""); 
        assert_eq!(vm.pc, ""); 
        assert_eq!(vm.mem, HashMap::new())
    }
}