use crate::vm::types::*;

struct Vm {
    program: Vec<Instruction>,
    call_stack: Vec<i32>,
    stack: Vec<i32>,
    current_block: usize,
    current_instr: usize,
}

impl Vm {
    pub fn new(program: Vec<Block>) -> Vm {
        Vm {
            stack: Vec::new(),
            program,
            call_stack: Vec::new(),
            current_block: 0,
            current_instr: 0,
        }
    }
    
    pub fn run(&mut self) {
        while self.current_instr < self.program.len() {
            match self.program[self.current_instr] {
                Instruction::Add { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] + &self.stack[src2];
                    self.current_instr += 1;
                }
                Instruction::Sub { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] - &self.stack[src2];
                    self.current_instr += 1;
                },
                Instruction::Mul { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] * &self.stack[src2];
                    self.current_instr += 1;
                },
                Instruction::Div { dst, src1, src2 } => {
                    self.stack[dst] = (&self.stack[src1] / &self.stack[src2]) as i32;
                    self.current_instr += 1;
                },
                Instruction::Copy { dst, src } => {
                    self.stack[dst] = self.stack[src].clone();
                    self.current_instr += 1;
                },
                Instruction::Push { val } => {
                    self.stack.push(val);
                    self.current_instr += 1;
                },
                Instruction::Del { src } => {
                    self.stack.remove(src);
                    self.current_instr += 1;
                },
                Instruction::Out { src } => {
                    println!("{}", &self.stack[src]);
                    self.current_instr += 1;
                },
                Instruction::OutStr { str } => {
                    println!("{}", str);
                    self.current_instr += 1;
                } 
                Instruction::Jump { dst } => {
                    self.current_instr = dst;
                },
                Instruction::JmpIfZero { dst, src } => {
                    if src == 0 {
                        self.current_instr = dst;
                    } else {
                        self.current_instr += 1;
                    }
                },
                Instruction::JmpIfNotZero { dst, src } => {
                    if src != 0 {
                        self.current_instr = dst;
                    } else {
                        self.current_instr += 1;
                    }
                },
                Instruction::JumpCmp { dst, els, cmp } => {
                    match cmp {
                        Comparison::Eq(src1, src2) => {
                            if self.stack[src1] == self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Neq(src1, src2) => {
                            if self.stack[src1] != self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Gt(src1, src2) => {
                            if self.stack[src1] > self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Lt(src1, src2) => {
                            if self.stack[src1] < self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Geq(src1, src2) => {
                            if self.stack[src1] >= self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Leq(src1, src2) => {
                            if self.stack[src1] <= self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                    }
                }
            }
        }
    }
}