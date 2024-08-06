pub type Value = f64;
type ValueArray = Vec<Value>;

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum OpCode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<i32>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new()
        }
    }
    pub fn disassemble_inst(&self, offset: usize){
        print!("{} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  ^ ");
        } else {
            print!("{} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OpCode::Return => {
                Chunk::simple_instr("OP_RETURN", offset)
            }
            OpCode::Constant(v) => {
                self.constant_instr("OP_CONSTANT", v)
            }
            OpCode::Negate => {
                Chunk::simple_instr("OP_NEGATE", offset)
            }
            OpCode::Add => {
                Chunk::simple_instr("OP_ADD", offset)
            }
            OpCode::Subtract => {
                Chunk::simple_instr("OP_SUBTRACT", offset)
            }
            OpCode::Multiply => {
                Chunk::simple_instr("OP_MULT", offset)
            }
            OpCode::Divide => {
                Chunk::simple_instr("OP_DIVIDE", offset)
            }
        }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for i in 0..self.code.len() {
            self.disassemble_inst(i);
        }
    }

    pub fn add_instr(&mut self, instr: OpCode, line: i32) {
        self.code.push(instr);
        self.lines.push(line);
    }



    pub fn add_constant(&mut self, constant: Value, line: i32) {
        self.constants.push(constant);
        self.add_instr(OpCode::Constant(constant), line);
    }

    fn constant_instr(&self, name: &str, val: Value) {
        println!("{} '{}'", name, val);

    }

    fn simple_instr(name: &str, offset: usize) {
        println!("{}", name);
    }
}

