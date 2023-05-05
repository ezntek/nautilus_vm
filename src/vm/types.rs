use crate::types::*;

#[derive(Clone, Copy)]
pub enum Comparison {
    Eq(usize, usize),
    Neq(usize, usize),
    Gt(usize, usize),
    Lt(usize, usize),
    Geq(usize, usize),
    Leq(usize, usize),
}

pub enum Instruction {
    Add { dst: usize, src1: usize, src2: usize },
    Sub { dst: usize, src1: usize, src2: usize },
    Mul { dst: usize, src1: usize, src2: usize },
    Div { dst: usize, src1: usize, src2: usize },
    Copy { dst: usize, src: usize },
    Push { val: i32 },
    Del { src: usize },
    Out { src: usize },
    OutStr { str: &'static str },
    Jump { dst: usize },
    JmpIfZero { dst: usize, src: usize },
    JmpIfNotZero { dst: usize, src: usize},
    JumpCmp { dst: usize, els: usize, cmp: Comparison },
}

pub struct Block<R>{
    instructions: Vec<Instruction>,
    return_value: Option<Box<dyn NlType<R>>>
}

impl<R> Block<R> {
    fn new(instructions: Vec<Instruction>) -> Block<R> {
        Block {
            instructions,
            return_value: None
        }
    }
} 