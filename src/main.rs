mod tsplcore;
mod basicassembly;

fn main() {
    println!("Hello, world!");

    // programma di prova
    let mut vm: tsplcore::Tsvm = tsplcore::tsvminit();
    // vm.mem.insert(String::from("start"), String::from("main^0"));
    // vm.mem
    //     .insert(String::from("main^0"), String::from("load^ciao"));
    // vm.mem
    //     .insert(String::from("main^1"), String::from("output"));
    // vm.mem.insert(String::from("main^2"), String::from("stop"));
    // tsplcore::execmain(vm);
    basicassembly::assemblyfromfile("tspl_examples/es1.txt", &mut vm);
    tsplcore::execmain(vm);
}
