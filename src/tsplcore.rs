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

pub fn pcnext(pc: &mut String) {
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
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = vm
            .mem
            .get(instructionlist[1])
            .expect("entry not found")
            .parse()
            .expect("not a number");
        let n3: i32 = n1 + n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "sub" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = vm
            .mem
            .get(instructionlist[1])
            .expect("entry not found")
            .parse()
            .expect("not a number");
        let n3: i32 = n1 - n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "and" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = vm
            .mem
            .get(instructionlist[1])
            .expect("entry not found")
            .parse()
            .expect("not a number");
        let n3: i32 = n1 & n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "or" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = vm
            .mem
            .get(instructionlist[1])
            .expect("entry not found")
            .parse()
            .expect("not a number");
        let n3: i32 = n1 | n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "not" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = !n1; // attenzione  ! = -(x + 1) -> !1 = -2 -> !0 = -1
        vm.acc = n2.to_string();
        pcnext(&mut vm.pc);
    } else if command == "compare" {
        let n1: i32 = vm.acc.parse().expect("not a number");
        let n2: i32 = vm
            .mem
            .get(instructionlist[1])
            .expect("entry not found")
            .parse()
            .expect("not a number");
        let n3: i32;
        if n2 == n1 {
            n3 = 0;
        } else if n1 > n2 {
            n3 = 1;
        } else {
            n3 = -1;
        }
        vm.acc = n3.to_string();
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
        } else {
            pcnext(&mut vm.pc);
        }
    }
    // default
    else if command == "stop" {
        vm.isrunning = false;
    } else {
        vm.isrunning = false;
    }
}

pub fn execmain(mut vm: &mut Tsvm, debug: bool) {
    vm.isrunning = true;
    vm.pc = String::from(&vm.mem["start"]);

    while vm.isrunning {
        exec(vm, debug);
    }
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
        assert_eq!(vm.isrunning, false);
        assert_eq!(vm.acc, "");
        assert_eq!(vm.pc, "");
        assert_eq!(vm.mem, HashMap::new())
    }

    // -- exec() tests
    // memory
    #[test]
    fn test_load() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("m^0"), String::from("load^sas"));
        exec(&mut vm, false);
        assert_eq!("sas", vm.acc)
    }

    #[test]
    fn test_get() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("var"), String::from("sas"));
        vm.mem.insert(String::from("m^0"), String::from("get^var"));
        exec(&mut vm, false);
        assert_eq!("sas", vm.acc)
    }

    #[test]
    fn test_set() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("sas");
        vm.mem.insert(String::from("m^0"), String::from("set^var"));
        exec(&mut vm, false);
        assert_eq!("sas", vm.mem.get("var").unwrap())
    }

    #[test]
    #[should_panic]
    fn test_del() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("var");
        vm.mem.insert(String::from("var"), String::from("sas"));
        vm.mem.insert(String::from("m^0"), String::from("del"));
        exec(&mut vm, false);
        vm.mem.get("var").unwrap();
    }

    // logic
    #[test]
    fn test_add() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("var"), String::from("2"));
        vm.mem.insert(String::from("m^0"), String::from("add^var"));
        exec(&mut vm, false);
        assert_eq!("3", vm.acc)
    }

    #[test]
    fn test_sub() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("var"), String::from("2"));
        vm.mem.insert(String::from("m^0"), String::from("sub^var"));
        exec(&mut vm, false);
        assert_eq!("-1", vm.acc)
    }

    #[test]
    fn test_and() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("var"), String::from("1"));
        vm.mem.insert(String::from("m^0"), String::from("and^var"));
        exec(&mut vm, false);
        assert_eq!("1", vm.acc); // 1 & 1

        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("var"), String::from("0"));
        exec(&mut vm, false);
        assert_eq!("0", vm.acc); // 1 & 0

        vm.pc = String::from("m^0");
        vm.acc = String::from("0");
        vm.mem.insert(String::from("var"), String::from("0"));
        exec(&mut vm, false);
        assert_eq!("0", vm.acc) // 0 & 0
    }

    #[test]
    fn test_or() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("var"), String::from("1"));
        vm.mem.insert(String::from("m^0"), String::from("or^var"));
        exec(&mut vm, false);
        assert_eq!("1", vm.acc); // 1 | 1

        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("var"), String::from("0"));
        exec(&mut vm, false);
        assert_eq!("1", vm.acc); // 1 | 0

        vm.pc = String::from("m^0");
        vm.acc = String::from("0");
        vm.mem.insert(String::from("var"), String::from("0"));
        exec(&mut vm, false);
        assert_eq!("0", vm.acc) // 0 | 0
    }

    #[test]
    fn test_not() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("m^0"), String::from("not"));
        exec(&mut vm, false);
        assert_eq!("-2", vm.acc); // !1

        vm.pc = String::from("m^0");
        vm.acc = String::from("0");
        exec(&mut vm, false);
        assert_eq!("-1", vm.acc) // !0
    }

    #[test]
    fn test_compare() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        vm.mem.insert(String::from("var"), String::from("2"));
        vm.mem.insert(String::from("m^0"), String::from("compare^var"));
        exec(&mut vm, false);
        assert_eq!("-1", vm.acc); // 1 > 2

        vm.pc = String::from("m^0");
        vm.acc = String::from("3");
        exec(&mut vm, false);
        assert_eq!("1", vm.acc); //  3 > 2

        vm.pc = String::from("m^0");
        vm.acc = String::from("2");
        exec(&mut vm, false);
        assert_eq!("0", vm.acc); // 2 == 2
    }

    // I/O ?

    //jumps
    #[test]
    fn test_jump() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("fn^0");
        vm.mem.insert(String::from("m^0"), String::from("jump"));
        exec(&mut vm, false);
        assert_eq!("fn^0", vm.pc)
    }

    #[test]
    fn test_jump0() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("0");
        vm.mem.insert(String::from("m^0"), String::from("jump0^fn^0"));
        exec(&mut vm, false);
        assert_eq!("fn^0", vm.pc);

        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        exec(&mut vm, false);
        assert_eq!("m^1", vm.pc)
    }

}
