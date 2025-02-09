#![allow(dead_code)]

use std::collections::HashMap;

// Shine's runtime

#[derive(Copy, Clone)]
pub enum DataType {
    None,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}
pub fn data_type_size(t: DataType) -> usize {
    use DataType::*;
    match t {
        None => 0,
        U8 | I8 => 1,
        U16 | I16 => 2,
        U32 | I32 | F32 => 4,
        U64 | I64 | F64 => 8,
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    EmitI64,
    EmitF64,

    NegI64,
    AddI64,
    SubI64,
    MulI64,
    DivI64,

    NegF64,
    AddF64,
    SubF64,
    MulF64,
    DivF64,

    //AddPtr,
    //SubPtr,

    //Pop,
    //PopN,

    //Deref,  // Pop ptr, read value at ptr, push value
    //DerefN,  // Pop n, pop ptr, read n values at ptr, push the n values

    //Set,
    //SetN,

    //PushFrame,  // Pop ptr, push to frame stack
    //PopFrame,  // Pop from frame stack
    //ReadFrame,  // Push top the top of the frame stack to the stack

    // ffi calls. Can do malloc, free, math, etc.
    //FnCall(fn(&mut Vec<Data>) -> ()),
}

#[derive(Copy, Clone)]
pub struct OpInfo {
    bytesize: u8,
    parts: [DataType; 3],
}

type TReg = u16;
const REG_TYPE: DataType = DataType::U16;
const REG_SIZE: u8 = 2;

pub fn op_info(op: OpCode) -> OpInfo {
    use OpCode::*;
    use DataType::*;
    match op {
        EmitI64 => OpInfo { bytesize: REG_SIZE + 8, parts: [REG_TYPE, I64, None] },  // rO, Value
        EmitF64 => OpInfo { bytesize: REG_SIZE + 8, parts: [REG_TYPE, I64, None] },  // rO, Value

        NegI64 => OpInfo { bytesize: REG_SIZE*2, parts: [REG_TYPE, REG_TYPE, None] },      // rO, rA
        AddI64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        SubI64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        MulI64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        DivI64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB

        NegF64 => OpInfo { bytesize: REG_SIZE*2, parts: [REG_TYPE, REG_TYPE, None] },      // rO, rA
        AddF64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        SubF64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        MulF64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
        DivF64 => OpInfo { bytesize: REG_SIZE*3, parts: [REG_TYPE, REG_TYPE, REG_TYPE] },  // rO, rA, rB
    }
}

pub fn read<T: bytemuck::Pod>(data: &[u8], location: usize) -> T {
    bytemuck::cast_slice::<u8, T>(&data[location..])[0]
}

pub fn eat<T: bytemuck::Pod>(data: &[u8], ip: &mut usize) -> T {
    let r = bytemuck::cast_slice::<u8, T>(&data[*ip..])[0];
    *ip += std::mem::size_of::<T>();
    r
}

pub fn write<T: bytemuck::Pod>(data: &mut [u8], location: usize, value: T) {
    bytemuck::cast_slice_mut::<u8, T>(&mut data[location..])[0] = value;
}

pub fn run_bytecode(code: &[u8], regs: &mut [u8]) {
    let mut ip: usize = 0;
    while ip < code.len() {
        let op: OpCode = unsafe { std::mem::transmute(eat::<u8>(code, &mut ip)) };
        use OpCode::*;
        match op {
            EmitI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let value = eat::<i64>(code, &mut ip);
                write(regs, ro, value);
            },
            EmitF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let value = eat::<f64>(code, &mut ip);
                write(regs, ro, value);
            },
            NegI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<i64>(regs, ra);
                write(regs, ro, -a);
            },
            AddI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<i64>(regs, ra);
                let b = read::<i64>(regs, rb);
                write(regs, ro, a + b);
            },
            SubI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<i64>(regs, ra);
                let b = read::<i64>(regs, rb);
                write(regs, ro, a - b);
            },
            MulI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<i64>(regs, ra);
                let b = read::<i64>(regs, rb);
                write(regs, ro, a * b);
            },
            DivI64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<i64>(regs, ra);
                let b = read::<i64>(regs, rb);
                write(regs, ro, a / b);
            },
            NegF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<f64>(regs, ra);
                write(regs, ro, -a);
            },
            AddF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<f64>(regs, ra);
                let b = read::<f64>(regs, rb);
                write(regs, ro, a + b);
            },
            SubF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<f64>(regs, ra);
                let b = read::<f64>(regs, rb);
                write(regs, ro, a - b);
            },
            MulF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<f64>(regs, ra);
                let b = read::<f64>(regs, rb);
                write(regs, ro, a * b);
            },
            DivF64 => {
                let ro = eat::<TReg>(code, &mut ip) as usize;
                let ra = eat::<TReg>(code, &mut ip) as usize;
                let rb = eat::<TReg>(code, &mut ip) as usize;
                let a = read::<f64>(regs, ra);
                let b = read::<f64>(regs, rb);
                write(regs, ro, a / b);
            },
        }
    }
}


//====================================================================================================

pub enum PlanTypeBase {
    I64,
    F64,
    Sprite(usize),

    // The variants below represent runtime types that coexist with a Sprite type.
    LabelVariant{base: usize, label: String},
    FrameVariant{base: usize, label: String, frame: usize},
}
pub struct PlanType {
    base: PlanTypeBase,
    ptr_depth: u64,
    is_mut: bool,
    is_pub: bool,
}

pub struct PlanMember {
    name: String,
    datatype: PlanType,
}

pub struct PlanSprite {
    name: String,
    strips: HashMap<String, PlanStrip>,
    members: Vec<PlanMember>,
}

pub struct PlanStrip {
    stepper: PlanStripStepper,
    keyframes: Vec<PlanKeyframe>,

    // I'm thinking inputs should be a part of the strip, rather than the keyframe. Not fully sure though, so this may change.

    // Might want to use an array, to specify an order.
    inputs: Vec<PlanMember>,
    members: Vec<PlanMember>,
}

pub struct PlanKeyframe {
    // Should inputs be a part of strips or a part of keyframes?
    // Ie., is there a use to letting each frame take potentially different inputs?
    // Maybe each frame can just take the previous frame as input? And maybe also the parent sprite?
    //inputs: HashMap<String, PlanType>,
    
    member: Vec<PlanMember>,
    steps: Vec<PlanInstruction>,
}

pub enum PlanStripStepper {
    NextFrameAndLoop,
    NextFrame,

    // Custom fn that chooses a new frame based on the sprite's state. Could it even use the parent's or stage's state?
    // Maybe I could pass in a closure (defined in Shine) that could use arbitrary data.
    //Custom(Fn(sprite state)->usize),
}

//==========

// Stmt? Expr? 
// Currently, it is just a sequence of "function calls".
pub enum PlanInstruction {
    MakeI64(i64),
    MakeF64(f64),
    Make2dPoint(usize, usize),
    
    Make2dLine(usize, usize),
    


    // Paths are quite high level. Let's try some simpler stuff for now.
    //MakePath(Box<MakePath>),
    //StrokePath(Box<StrokePath>),
    //FillPath(Box<FillPath>),
}

// pub struct MakePath {}
// pub struct StrokePath {
//     path: usize,  // points to path data
// }
// pub struct FillPath {
//     path: usize,
// }

//==========

pub enum Sprite {
    Standard(StandardSprite),  // Unoptimized. Can always be used.
}

pub struct StandardSprite {
    labeled_states: HashMap<String, Vec<Keyframe>>,
    current_label: String,
    current_keyframe: usize,  // Should keyframes be persisted when in a different label state? I think not. If you switch to a new state, then you are in a new state.
}

pub struct Keyframe {
    // data
    data: HashMap<String, Thing>,
    // graphics
}

pub enum Thing {
    I64(i64),
    F64(f64),
    Str(String),  // Should this be owned? Probably?
    //Sprite(),
    //Ptr(usize),
}

// pub struct Thing {
//     value: ThingValue,
//     ptr_depth: u32,  // ptr_depth is a plan type thing.
// }

struct Stage {
    root_plan: PlanSprite,
    root: Sprite,  // Make this optional? As in, it is not always executing? Could also have a list of active sprites and plans.
    fps: f64,
}



pub fn step_stage(stage: &mut Stage) {
    step_sprite(&stage.root_plan, &mut stage.root);
}

pub fn step_sprite(plan: &PlanSprite, sprite: &mut Sprite) {
    // I need the plan here. The plan is the "instructions" to be executed.
    // The Sprite is just the data.
    match sprite {
        Sprite::Standard(standard_sprite) => {
            standard_sprite.labeled_states[standard_sprite.current_label];
        }
    }
}