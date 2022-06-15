pub mod tsplcore;

fn main() {
    println!("Hello, world!");

    // programma di prova
    let mut vm: tsplcore::Tsvm = tsplcore::tsvminit();
    vm.mem.insert(String::from("start"), String::from("main^0"));
    vm.mem
        .insert(String::from("main^0"), String::from("load^ciao"));
    vm.mem
        .insert(String::from("main^1"), String::from("output"));
    vm.mem.insert(String::from("main^2"), String::from("stop"));
    tsplcore::execmain(vm);
}
