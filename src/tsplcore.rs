use std::{collections::HashMap, cmp::Ordering};

pub struct Tsvm {
    pub isrunning: bool,
    pub acc: String,
    pub pc: String,
    pub mem: HashMap<String, String>,
    pub stack: Vec<String>,
    pub iter: Vec<String>
}

pub fn tsvminit() -> Tsvm {
    Tsvm {
        isrunning: false,
        acc: String::from(""),
        pc: String::from(""),
        mem: HashMap::new(),
        stack: vec![String::from("")],
        iter: Vec::new(),
    }
}

pub fn pcnext(pc: &mut String) {
    let v: Vec<&str> = pc.splitn(2, '^').collect();
    let mut n: i32 = v[1].parse().unwrap_or_else(|_| {panic!("pc = {} is not in a right format", pc)});
    n += 1;
    let name: String = String::from(v[0]);
    let num: String = n.to_string();
    pc.clear();
    pc.push_str(&name);
    pc.push('^');
    pc.push_str(&num);
}

pub fn rncommand(command: &str) -> String {
    let v: Vec<&str> = command.splitn(2, '^').collect();
    String::from(v[0])
}

fn crashreport(vm: &Tsvm) {
    println!("################");
    println!("CRASH REPORT");
    println!("call stack = {:?}", vm.stack);
    println!("pc = {}, acc = {}", vm.pc, vm.acc);
    println!("memory = {:?}", vm.mem);
    println!("################");
}

fn exec(vm: &mut Tsvm, debug: bool) {
    if debug {
        println!("DB pc: {}", vm.pc);
    }
    let instruction: String = vm
        .mem
        .get(&vm.pc)
        .unwrap_or_else(|| {crashreport(vm); panic!("not found entry for pc = {}", vm.pc)})
        .clone();

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
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);

        vm.acc = String::from(&vm.mem[&namefrom]);
        pcnext(&mut vm.pc);
    } else if command == "set" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        vm.mem
            .insert(namefrom, vm.acc.clone());
        pcnext(&mut vm.pc);
    } else if command == "del" {
        vm.mem.remove(&vm.acc);
        pcnext(&mut vm.pc);
    }
    // logic
    else if command == "append" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);

        vm.acc.push_str(vm.mem.get(&namefrom).expect("entry not found"));
        pcnext(&mut vm.pc);
    }
    else if command == "getnext" {
        vm.acc = vm.iter.remove(0);
        pcnext(&mut vm.pc);
    }
    else if command == "add" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 + n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "sub" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 - n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "mult" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 * n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "div" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 / n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "and" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 & n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "or" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        let n3: i32 = n1 | n2;
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    } else if command == "not" {
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = !n1; // attenzione  ! = -(x + 1) -> !1 = -2 -> !0 = -1
        vm.acc = n2.to_string();
        pcnext(&mut vm.pc);
    } else if command == "compare" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        
        let n1: i32 = vm.acc.parse().unwrap_or_else(|_| {crashreport(vm); panic!("{} not a number", vm.acc)});
        let n2: i32 = vm
            .mem
            .get(&namefrom)
            .expect("entry not found")
            .parse()
            .unwrap_or_else(|_| {crashreport(vm);panic!("{} not a number", namefrom)});
        
        // clippy?
        let n3: i32 = match n1.cmp(&n2) {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => -1,
        };
        vm.acc = n3.to_string();
        pcnext(&mut vm.pc);
    }
    else if command == "split" {
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);
        vm.iter.clear();
        for token in vm.acc.split(vm.mem.get(&namefrom).expect("entry not found")) {
            vm.iter.push(token.to_string());
        }
        pcnext(&mut vm.pc);
    }
    else if command == "splitall" {
        vm.iter.clear();
        for token in vm.acc.chars() {
            vm.iter.push(token.to_string());
        }
        pcnext(&mut vm.pc);
    }
    // I/O
    else if command == "input" {
        vm.acc = "".to_string();
        std::io::stdin().read_line(&mut vm.acc).unwrap();
        vm.acc = String::from(vm.acc.trim());
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
    // functions
    else if command == "call" {
        let mut namespace: String = rncommand(instructionlist[1]);
        namespace.push('$');
        while vm.stack.contains(&namespace) { //if recursion
            namespace.push('*');
        }
        vm.stack.push(namespace.clone());

        let mut returnpc: String = namespace;
        returnpc.push_str("_ret");

        vm.mem.insert(returnpc, vm.pc.clone());

        vm.pc = String::from(instructionlist[1]);
    }
    else if command == "uncall" {
        
        let namespace: String = vm.stack.pop().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")});

        let mut returnpc: String = namespace;
        returnpc.push_str("_ret");

        vm.pc = String::from(vm.mem.get(&returnpc).unwrap_or_else(|| {crashreport(vm); panic!("{} not a return point", returnpc)}));
        vm.mem.remove(&returnpc);

        pcnext(&mut vm.pc);
    }
    else if command == "pass" {
        let mut namefrom: String = vm.stack.get(vm.stack.len() - 2).unwrap_or_else(|| {crashreport(vm); panic!("call stack has len < 2")}).clone();
        namefrom.push_str(instructionlist[1]);
        let mut nameto: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        nameto.push_str(instructionlist[1]);

        vm.mem.insert(nameto, String::from(vm.mem.get(&namefrom).unwrap_or_else(|| {crashreport(vm); panic!("not found entry for pc = {}", namefrom)})));
        pcnext(&mut vm.pc);
    }
    else if command == "return" {
        let mut nameto: String = vm.stack.get(vm.stack.len() - 2).unwrap_or_else(|| {crashreport(vm); panic!("call stack has len < 2")}).clone();
        nameto.push_str(instructionlist[1]);
        let mut namefrom: String = vm.stack.last().unwrap_or_else(|| {crashreport(vm); panic!("call stack has len zero")}).clone();
        namefrom.push_str(instructionlist[1]);

        vm.mem.insert(nameto, String::from(vm.mem.get(&namefrom).unwrap_or_else(|| {crashreport(vm); panic!("not found entry for pc = {}", namefrom)})));
        pcnext(&mut vm.pc);
    }
    // default
    else if command == "stop" {
        vm.isrunning = false;
    } else {
        crashreport(vm);
        panic!("{} is not a command", command);
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
        assert!(!vm.isrunning);
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
    fn test_append() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("ciao ");
        vm.mem.insert(String::from("var"), String::from("belli"));
        vm.mem.insert(String::from("m^0"), String::from("append^var"));
        exec(&mut vm, false);
        assert_eq!("ciao belli", vm.acc)
    }

    #[test]
    fn test_split() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("1^23^456");
        vm.mem.insert(String::from("var"), String::from("^"));
        vm.mem.insert(String::from("m^0"), String::from("split^var"));
        exec(&mut vm, false);
        assert_eq!(vec!["1", "23", "456"], vm.iter)
    }

    #[test]
    fn test_splitall() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("abc");
        vm.mem.insert(String::from("m^0"), String::from("splitall"));
        exec(&mut vm, false);
        assert_eq!(vec!["a", "b", "c"], vm.iter)
    }

    #[test]
    fn test_getnext() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.iter = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
        vm.mem.insert(String::from("m^0"), String::from("getnext"));
        vm.mem.insert(String::from("m^1"), String::from("getnext"));
        exec(&mut vm, false);
        assert_eq!("aa", vm.acc);
        exec(&mut vm, false);
        assert_eq!("bb", vm.acc)
    }

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
    fn test_mult() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("3");
        vm.mem.insert(String::from("var"), String::from("2"));
        vm.mem.insert(String::from("m^0"), String::from("mult^var"));
        exec(&mut vm, false);
        assert_eq!("6", vm.acc)
    }

    #[test]
    fn test_div() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.acc = String::from("12");
        vm.mem.insert(String::from("var"), String::from("2"));
        vm.mem.insert(String::from("m^0"), String::from("div^var"));
        exec(&mut vm, false);
        assert_eq!("6", vm.acc)
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
        vm.mem
            .insert(String::from("m^0"), String::from("compare^var"));
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
        vm.mem
            .insert(String::from("m^0"), String::from("jump0^fn^0"));
        exec(&mut vm, false);
        assert_eq!("fn^0", vm.pc);

        vm.pc = String::from("m^0");
        vm.acc = String::from("1");
        exec(&mut vm, false);
        assert_eq!("m^1", vm.pc)
    }

    #[test]
    fn test_input() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("m^0"), String::from("input"));
        println!("type yes");
        exec(&mut vm, false);
        assert_eq!("yes", vm.acc)
    }

    #[test]
    fn test_call() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("m^0"), String::from("call^fn^0"));
        vm.mem.insert(String::from("fn^0"), String::from("load^1"));
        exec(&mut vm, false);
        assert_eq!("fn$", vm.stack.last().unwrap());
        assert_eq!("m^0", vm.mem.get("fn$_ret").unwrap());
        assert_eq!("fn^0", vm.pc);
    }

    #[test]
    fn test_uncall() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("m^0"), String::from("call^fn^0"));
        vm.mem.insert(String::from("fn^0"), String::from("uncall"));
        exec(&mut vm, false);
        exec(&mut vm, false);
        assert_eq!("", vm.stack.last().unwrap());
        assert_eq!("m^1", vm.pc);
    }

    #[test]
    fn test_pass() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("var"), String::from("ciao"));
        vm.mem.insert(String::from("m^0"), String::from("call^fn^0"));
        vm.mem.insert(String::from("fn^0"), String::from("pass^var"));
        exec(&mut vm, false);
        exec(&mut vm, false);
        assert_eq!("ciao", vm.mem.get("fn$var").unwrap())
    }

    #[test]
    fn test_return() {
        let mut vm: Tsvm = tsvminit();
        vm.pc = String::from("m^0");
        vm.mem.insert(String::from("fn$var"), String::from("bello"));
        vm.mem.insert(String::from("m^0"), String::from("call^fn^0"));
        vm.mem.insert(String::from("fn^0"), String::from("return^var"));
        exec(&mut vm, false);
        exec(&mut vm, false);
        assert_eq!("bello", vm.mem.get("var").unwrap())
    }

}
