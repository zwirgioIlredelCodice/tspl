use std::env;

mod basicassembly;
mod tsplcore;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut vm: tsplcore::Tsvm = tsplcore::tsvminit();
    basicassembly::assembler(filename, &mut vm);
    tsplcore::execmain(&mut vm, false);
}