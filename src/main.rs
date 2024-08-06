use std::env;
use std::process::exit;
use crate::op_code::{Chunk, OpCode};
use crate::machine::*;
mod op_code;
mod machine;
mod compiler;
mod token;

fn main() {
    let mut vm = VirtualMachine::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 { panic!("Wrong arguments") }
    else if args.len() == 2 {
        vm.run_file(&args[1]);
    } else {
        vm.repl();
    }
}
