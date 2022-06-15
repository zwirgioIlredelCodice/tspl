use std::collections::HashMap;

struct Tsvm {
    isrunning: bool,
    acc: String,
    pc:  String,
    mem: HashMap<String, String>,
}

fn pcnext(pc: String) -> String {
    let v: Vec<&str> = pc.split('^').collect();
    let mut n: i32 = v[1].parse().unwrap();
    n += 1;
    let mut name: String = String::from(v[0]);
    let num: String = n.to_string();
    name.push('^');
    name.push_str(&num);
    name
}

fn exec(mut vm: Tsvm) -> Tsvm {
    let instruction: String = vm.mem[&vm.pc].clone();

    //parse instruction
    let instructionlist: Vec<&str> = instruction.split('^').collect();
    let command: &str = instructionlist[0];

    // memory
    if command == "load" {
        vm.acc = String::from(instructionlist[1]);
    } else if command == "get" {
        vm.acc = String::from(&vm.mem[instructionlist[1]]);
    } else if command == "set" {
        vm.mem.insert(String::from(instructionlist[1]), vm.acc.clone());
    } else if command == "del" {
        vm.mem.remove(&vm.acc);
    } 

    // logic
    else if command == "add" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = String::from(instructionlist[1]).parse().unwrap();
        let n3: i32 = n1 + n2;
        vm.acc = n3.to_string();
    } else if command == "sub" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = String::from(instructionlist[1]).parse().unwrap();
        let n3: i32 = n1 - n2;
        vm.acc = n3.to_string();
    } else if command == "and" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = String::from(instructionlist[1]).parse().unwrap();
        let n3: i32 = n1 & n2;
        vm.acc = n3.to_string();
    } else if command == "or" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = String::from(instructionlist[1]).parse().unwrap();
        let n3: i32 = n1 | n2;
        vm.acc = n3.to_string();
    } else if command == "not" {
        let n1: i32 = vm.acc.parse().unwrap();
        let n2: i32 = ! n1;
        vm.acc = n2.to_string();
    }

    // I/O
    else if command == "input" {
        std::io::stdin().read_line(&mut vm.acc).unwrap();
    } else if command == "output" {
        print!("{}", vm.acc);
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
    } else  {
        vm.isrunning = false;
    }
    vm
}

fn execmain(mut vm: Tsvm) -> Tsvm {
    vm.isrunning = true;
    vm.pc = String::from(&vm.mem["start"]);

    while vm.isrunning {
        vm = exec(vm);
        vm.pc = pcnext(vm.pc);
    }

    vm
}

fn tsvminit() -> Tsvm {
    Tsvm { 
            isrunning: false, 
            acc: String::from(""),
            pc: String::from(""), 
            mem: HashMap::new() 
        }
}

fn main() {
    println!("Hello, world!");
    
    // programma di prova
    let mut vm:Tsvm = tsvminit();
    vm.mem.insert(String::from("start"), String::from("main^0"));
    vm.mem.insert(String::from("main^0"), String::from("load^ciao"));
    vm.mem.insert(String::from("main^1"), String::from("output"));
    vm.mem.insert(String::from("main^2"), String::from("stop"));
    execmain(vm);
}
