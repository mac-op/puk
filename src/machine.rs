use std::fs;
use std::io::Write;
use std::process::exit;
use crate::compiler::{Compiler, Scanner};
use crate::op_code::{Chunk, OpCode, Value};

pub enum InterpretResult {
    OK, CompileErr, RuntimeErr
}


pub struct VirtualMachine {
    pub curr_chunk: Option<Chunk>,
    instruction_ptr: i64,
    compiler: Compiler,
    stack: Vec<Value>
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            curr_chunk: None,
            instruction_ptr: -1,
            compiler: Compiler{},
            stack: Vec::new()
        }
    }

    pub fn interpret(& mut self, content: & str) -> InterpretResult {
        self.curr_chunk = Some(Chunk::new());
        if matches!(self.compiler.compile(content,self.curr_chunk.as_mut().unwrap()), InterpretResult::CompileErr) {
            return InterpretResult::CompileErr;
        }
        self.instruction_ptr = 0;
        self.run()
    }

    fn binary_op(&mut self, op: &OpCode) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result: Value;
        match op {
            OpCode::Add => { result = a + b; }
            OpCode::Subtract => { result = a - b; }
            OpCode::Multiply => { result = a * b; }
            OpCode::Divide => { result = a / b; }
            _ => { unreachable!() }
        }

        self.stack.push(result);
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let pos = self.instruction_ptr as usize;
            let current_op = self.curr_chunk.as_ref().unwrap().code[pos];
            match current_op {
                OpCode::Constant(v) => {
                    println!("{v}");
                    self.stack.push(v);
                }
                OpCode::Return => {
                    let v = self.stack.pop().unwrap();
                    println!("{v}");
                    return InterpretResult::OK;
                }
                OpCode::Negate => {
                    let top = self.stack.pop().unwrap();
                    self.stack.push(-top);
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.binary_op(& current_op);
                }
            }

            self.instruction_ptr += 1;
        }
    }

    pub fn run_file(&mut self, source: &str) {
        let content = fs::read_to_string(source).expect("Cannot read file");
        let result = self.interpret(&content);

        match result {
            InterpretResult::OK => {return;}
            InterpretResult::CompileErr => {exit(99);}
            InterpretResult::RuntimeErr => {exit(11);}
        }
    }

    pub fn repl(&mut self) {
        let mut line = String::new();
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let read = std::io::stdin().read_line(& mut line).unwrap();
            if read == 0 {
                println!();
                break;
            }
            self.interpret(& mut line);
            line.clear();
        }
    }
}
