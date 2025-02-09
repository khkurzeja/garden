use std::collections::HashMap;
use std::ops::Range;

// A second attempt at the runtime.
// This one aims for simplicity. As such, it is dynamically typed.

type TConstIdx = u16;  // Allows 64k consts
type TReg = u16;  // Allows 64k registers
const REG_SIZE: u8 = 2;

// Can I have both dynamic typing and explicit control over memory layout?
// Let's assume that a variable cannot be "redeclared".


// Dynamic value
#[derive(Copy, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    I64(i64),
    F64(f64),
    Dictionary(*mut HashMap<String, Value>),  // Might want to use a ptr instead, so as to not prescribe memory ownership.
    Array(*mut [Value]),
    ValuePtr(*mut Value),
    //ValueUniquePtr(*mut Value),  // This ptr owns the memory, and frees the memory when it goes out of scope.

    CommandList(*const CommandList),

    //DynamicArray(*mut Vec<Value>),
    //Ptr(*mut u8),  // Or ref?
    //Function(??),  // Or block?
    
    // Potential future stuff below
    // Type(),
}


//====================================================================================================
// Ast

// Should I make a dynamic verion of the block system?
// Ie. a block takes inputs then runs code that generates data, some of which is output.

// I should keep the Flash stuff out of this layer. Rather, the Flash stuff should be implemented
// using this layer. This will allow me to implement alternative systems using this layer, if I 
// find that the Flash ideas don't pan out.

pub struct Argument {
    name: String,
    value_id: usize,
}

// What is a better word?
pub enum Command {
    // Stmt
    Set{ref_id: usize, value_id: usize},

    // Expr
    Lit{reg: usize, value: Value},
    Me{reg: usize},  // Self, This, etc. (How should this be implemented. Should id==0 represent Me?)
    Deref{reg: usize, ptr_id: usize},  // Deref the value pointed at
    
    MakeArray{reg: usize, size_id: usize},  // Dynamic type.
    RefArray{reg: usize, array_id: usize, index_id: usize},
    SizeArray{reg: usize, array_id: usize},

    RefCommandList{reg: usize, name: String},

    Make{reg: usize, command_list_id: usize, args: Range<usize>},
}

// Defines a block type. Basically, a more compact Ast rep.
pub struct CommandList {
    last_id: usize,
    pub data: Vec<Command>,
    pub args_buffer: Vec<Argument>,
    pub params: Vec<String>,
    pub members: Vec<String>,
    // I could have an array of String, if I wanted to let the commands use str.
    // I will just let Commands take String for now though.
    // I could also have a list of arguments to make blocks here, so I don't need n different lists for n make calls.
}
impl CommandList {
    pub fn new() -> Self {
        Self { last_id: 0, data: vec![], args_buffer: vec![], params: vec![], members: vec![] }
    }

    fn eat_id(&mut self) -> usize {
        self.last_id += 1;
        self.last_id
    }

    pub fn decl_param(&mut self, name: String) {
        self.params.push(name);
    }
    pub fn decl(&mut self, name: String) {
        self.members.push(name);
    }
    pub fn set(&mut self, ref_id: usize, value_id: usize) {
        self.data.push(Command::Set{ref_id, value_id});
    }

    pub fn lit(&mut self, value: Value) -> usize {
        self.data.push(Command::Lit{reg: self.eat_id(), value});
        self.last_id
    }
    pub fn me(&mut self) -> usize {
        self.data.push(Command::Me{reg: self.eat_id()});
        self.last_id
    }
    pub fn deref(&mut self, ptr_id: usize) -> usize {
        self.data.push(Command::Deref{reg: self.eat_id(), ptr_id});
        self.last_id
    }
    pub fn make_array(&mut self, size_id: usize) -> usize {
        self.data.push(Command::MakeArray{reg: self.eat_id(), size_id});
        self.last_id
    }
    pub fn ref_array(&mut self, array_id: usize, index_id: usize) -> usize {
        self.data.push(Command::RefArray{reg: self.eat_id(), array_id, index_id});
        self.last_id
    }
    pub fn size_array(&mut self, array_id: usize) -> usize {
        self.data.push(Command::SizeArray{reg: self.eat_id(), array_id});
        self.last_id
    }

    pub fn ref_command_list(&mut self, name: String) -> usize {
        self.data.push(Command::RefCommandList{reg: self.eat_id(), name});
        self.last_id
    }
    pub fn make(&mut self, command_list_id: usize, args: Vec<Argument>) -> usize {
        let start = self.args_buffer.len();
        for arg in args { self.args_buffer.push(arg); }
        let end = self.args_buffer.len();
        self.data.push(Command::Make{reg: self.eat_id(), command_list_id, args: start..end});
        self.last_id
    }
}

pub struct CommandSet {
    pub lists: HashMap<String, CommandList>,
}
impl CommandSet {
    pub fn new() -> Self { Self{ lists: HashMap::new() } }
}

struct Environment {
    pub set: CommandSet,
    pub arrays: Vec<Vec<Value>>,
}
impl Environment {
    pub fn new() -> Self {
        Self { set: CommandSet::new(), arrays: vec![], }
    }

    pub fn alloc_array(&mut self, size: usize) -> usize {
        self.arrays.push(vec![Value::Nil; size]);
        self.arrays.len() - 1
    }

    pub fn run(&self, name: String, args: &HashMap<String, Value>) {
        let list = &self.set.lists[&name];

        // Stack must be static size since we want to support ref types.
        // Alternatively, could alloc a new stack for every call. Fixed size is good for now.
        const STACK_SIZE: usize = 1024*1024;
        let mut stack = [Value::Nil; STACK_SIZE];
        let mut sp: usize = 0;

        for i in 0..list.params.len() {
            stack[sp] = args[&list.params[i]];
            sp += 1;
        }

        //**TODO: Ideally, the reg-based commands will have been optimized by now (compressed and reused)

        let mut frame_ptr: usize = 0;
        let mut ip: usize = 0;
        use Command::*;
        for i in 0..list.data.len() {
            match list.data[i] {
                Set{ref_id, value_id} => {
                    let value = stack[value_id];
                    let loc = stack[ref_id];
                    match loc {
                        Value::ValuePtr(ptr) => unsafe { *ptr = value },
                        _ => { assert!(false); },
                    }
                },
                Lit{reg, value} => {
                    stack[reg] = value;
                },
                Me{reg} => {
                    stack[reg] = stack[frame_ptr];
                },
                Deref{reg, ptr_id} => {
                    let loc = stack[ptr_id];
                    match loc {
                        Value::ValuePtr(ptr) => stack[reg] = unsafe { *ptr },
                        _ => { assert!(false); },
                    }
                },
                MakeArray{reg, size_id} => {
                    let size_val = stack[size_id];
                    match size_val {
                        Value::I64(value) => {
                            //**TODO!!!!!!!!!!!!!!!!!!!!
                            stack[reg] = 
                        },
                        _ => { assert!(false); },
                    }
                },
                RefArray{reg, array_id, index_id} => {
                    let index_value = stack[index_id];
                    let array_value = stack[array_id];
                    match (index_value, array_value) {
                        (Value::I64(index), Value::Array(array_ptr)) => {
                            stack[reg] = unsafe{*array_ptr}[index as usize];
                        },
                        _ => { assert!(false); },
                    }
                },
                SizeArray{reg, array_id} => {
                    let array_value = stack[array_id];
                    match array_value {
                        Value::Array(array_ptr) => {
                            stack[reg] = Value::I64(unsafe{*array_ptr}.len() as i64);
                        },
                        _ => { assert!(false); },
                    }
                },

                RefCommandList{reg, name} => {
                    let command_list = &self.set.lists[&name];
                    stack[reg] = Value::CommandList(command_list)
                },

                Make{reg, command_list_id, args} => {
                    //**TODO: In correct order, 
                    // make new dictionary, push on stack
                    // Update frame_ptr to point at new dictionary
                    // push args on stack
                    // run constructor code
                },
            }
        }


        //**TODO: Return stuff
    }
}


//====================================================================================================
// System

// Sprite has States (dictionary)
// States have keystrips (array)
// Keystrips have keyframes (dictionary)
// Keyframes have data and code, including but not limited to graphics and interaction

// Why use this architecture? Other than its obvious mapping to Flash?
// Maybe the mapping to Flash is a good reason, for a first prototype.
// I can think about how to make things better once I have something reasonable and familiar working.

// Based on its contents, a sprite may compile into
// struct  (todo: explain how)
// enum
// function
// ...




//====================================================================================================
// Old VM

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    Emit,  // TReg, TConstIdx

    Neg,  // TReg, TReg
    Add,  // TReg, TReg, TReg
    Sub,  // TReg, TReg, TReg
    Mul,  // TReg, TReg, TReg
    Div,  // TReg, TReg, TReg
    Mod,  // TReg, TReg, TReg

    BitNot,  // TReg, TReg
    BitAnd,  // TReg, TReg, TReg
    BitOr,   // TReg, TReg, TReg
    BitXor,  // TReg, TReg, TReg
    BitShiftLeft,  // TReg, TReg, TReg
    BitShiftRight,  // TReg, TReg, TReg

    Not,        // TReg, TReg
    And,        // TReg, TReg, TReg
    Or,         // TReg, TReg, TReg
    Eq,         // TReg, TReg, TReg
    NotEq,      // TReg, TReg, TReg
    Less,       // TReg, TReg, TReg
    LessEq,     // TReg, TReg, TReg
    Greater,    // TReg, TReg, TReg
    GreaterEq,  // TReg, TReg, TReg
}


pub fn eat<T: bytemuck::Pod>(data: &[u8], ip: &mut usize) -> T {
    let r = bytemuck::cast_slice::<u8, T>(&data[*ip..])[0];
    *ip += std::mem::size_of::<T>();
    r
}
// pub fn read<T: bytemuck::Pod>(data: &[u8], location: usize) -> T {
//     bytemuck::cast_slice::<u8, T>(&data[location..])[0]
// }
// pub fn write<T: bytemuck::Pod>(data: &mut [u8], location: usize, value: T) {
//     bytemuck::cast_slice_mut::<u8, T>(&mut data[location..])[0] = value;
// }

pub fn run_bytecode(code: &[u8], consts: &[Value], regs: &mut [Value]) {
    use Value::*;
    let mut ip: usize = 0;
    while ip < code.len() {
        let op: OpCode = unsafe { std::mem::transmute(eat::<u8>(code, &mut ip)) };
        use OpCode::*;
        match op {
            Emit => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let index = eat::<TConstIdx>(code, &mut ip);
                regs[ro] = consts[index as usize];
            },
            Neg => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                match regs[ra] {
                    I64(v) => regs[ro] = I64(-v),
                    F64(v) => regs[ro] = F64(-v),
                    _ => panic!(),
                }
            },
            Add => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a + b),
                    (F64(a), F64(b)) => regs[ro] = F64(a + b),
                    _ => panic!(),
                }
            },
            Sub => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a - b),
                    (F64(a), F64(b)) => regs[ro] = F64(a - b),
                    _ => panic!(),
                }
            },
            Mul => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a * b),
                    (F64(a), F64(b)) => regs[ro] = F64(a * b),
                    _ => panic!(),
                }
            },
            Div => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a / b),
                    (F64(a), F64(b)) => regs[ro] = F64(a / b),
                    _ => panic!(),
                }
            },
            Mod => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a % b),
                    (F64(a), F64(b)) => regs[ro] = F64(a % b),
                    _ => panic!(),
                }
            },

            BitNot => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                match regs[ra] {
                    I64(v) => regs[ro] = I64(!v),
                    _ => panic!(),
                }
            },
            BitAnd => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a & b),
                    _ => panic!(),
                }
            },
            BitOr => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a | b),
                    _ => panic!(),
                }
            },
            BitXor => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a ^ b),
                    _ => panic!(),
                }
            },
            BitShiftLeft => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a << b),
                    _ => panic!(),
                }
            },
            BitShiftRight => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = I64(a >> b),
                    _ => panic!(),
                }
            },

            Not => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                match regs[ra] {
                    Bool(a) => regs[ro] = Bool(!a),
                    _ => panic!(),
                }
            },
            And => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (Bool(a), Bool(b)) => regs[ro] = Bool(a && b),
                    _ => panic!(),
                }
            },
            Or => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (Bool(a), Bool(b)) => regs[ro] = Bool(a || b),
                    _ => panic!(),
                }
            },
            Eq => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (Bool(a), Bool(b)) => regs[ro] = Bool(a == b),
                    (I64(a), I64(b))   => regs[ro] = Bool(a == b),
                    (F64(a), F64(b))   => regs[ro] = Bool(a == b),
                    _ => panic!(),
                }
            },
            NotEq => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (Bool(a), Bool(b)) => regs[ro] = Bool(a != b),
                    (I64(a), I64(b))   => regs[ro] = Bool(a != b),
                    (F64(a), F64(b))   => regs[ro] = Bool(a != b),
                    _ => panic!(),
                }
            },
            Less => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = Bool(a < b),
                    (F64(a), F64(b)) => regs[ro] = Bool(a < b),
                    _ => panic!(),
                }
            },
            LessEq => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = Bool(a <= b),
                    (F64(a), F64(b)) => regs[ro] = Bool(a <= b),
                    _ => panic!(),
                }
            },
            Greater => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = Bool(a > b),
                    (F64(a), F64(b)) => regs[ro] = Bool(a > b),
                    _ => panic!(),
                }
            },
            GreaterEq => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                match (regs[ra], regs[rb]) {
                    (I64(a), I64(b)) => regs[ro] = Bool(a >= b),
                    (F64(a), F64(b)) => regs[ro] = Bool(a >= b),
                    _ => panic!(),
                }
            },
        }
    }
}