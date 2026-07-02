use crate::isa::{decode, Op};


pub struct Vm {
    stack: Vec<i64>,
    globals: [i64; 256],
    ip: usize,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            globals: [0; 256],
            ip: 0,
        }
    }

    
    fn push(&mut self, val: i64) -> Result<(), String> {
        if self.stack.len() >= 1024 {
            return Err(format!(
                "trap at ip=0x{:04X}: stack overflow", self.ip
            ));
        }
        self.stack.push(val);
        Ok(())
    }

    
    fn pop(&mut self) -> Result<i64, String> {
        self.stack.pop().ok_or_else(|| format!(
            "trap at ip=0x{:04X}: stack underflow", self.ip
        ))
    }

    
    pub fn run(&mut self, code: &[u8]) -> Result<(), String> {
    
        self.ip = 0;

        loop {
            
            if self.ip >= code.len() {
                return Err(format!(
                    "trap at ip=0x{:04X}: ip past end without HALT", self.ip
                ));
            }
            let (op, size) = decode(code, self.ip);

            self.ip += size;
            match op {
                
                Op::Push(n) => { self.push(n)?; }

                
                Op::Pop => { self.pop()?; }

            
                Op::Dup => {
                    let top = self.pop()?;
                    self.push(top)?;
                    self.push(top)?;
                }

                
                Op::Swap => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(b)?;
                    self.push(a)?;
                }

                
                Op::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                }
                Op::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                }
                Op::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b)?;
                }

                
                Op::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err(format!(
                            "trap at ip=0x{:04X}: division by zero", self.ip
                        ));
                    }
                    self.push(a / b)?;
                }

                
                Op::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err(format!(
                            "trap at ip=0x{:04X}: mod by zero", self.ip
                        ));
                    }
                    self.push(a % b)?;
                }

                
                Op::Neg => {
                    let a = self.pop()?;
                    self.push(-a)?;
                }

                
                Op::Load(slot) => {
                    let val = self.globals[slot as usize];
                    self.push(val)?;
                }

                
                Op::Store(slot) => {
                    let val = self.pop()?;
                    self.globals[slot as usize] = val;
                }

                
                Op::Print => {
                    let val = self.pop()?;
                    println!("{}", val);
                }

                
                Op::Halt => {
                    return Ok(());
                }
            }
        }
    }

    
    pub fn run_trace(&mut self, code: &[u8]) -> Result<(), String> {
        self.ip = 0;

        loop {
            if self.ip >= code.len() {
                return Err(format!(
                    "trap at ip=0x{:04X}: ip past end without HALT", self.ip
                ));
            }

            // decode what instruction is coming next
            let (op, size) = decode(code, self.ip);

            // print current state BEFORE executing
            println!(
                "ip=0x{:04X}  {:?}  stack={:?}",
                self.ip, op, self.stack
            );

            
            self.ip += size;

            
            match op {
                Op::Push(n) => { self.push(n)?; }
                Op::Pop     => { self.pop()?; }

                Op::Dup => {
                    let top = self.pop()?;
                    self.push(top)?;
                    self.push(top)?;
                }

                Op::Swap => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(b)?;
                    self.push(a)?;
                }

                Op::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                }
                Op::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                }
                Op::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b)?;
                }

                Op::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err(format!(
                            "trap at ip=0x{:04X}: division by zero", self.ip
                        ));
                    }
                    self.push(a / b)?;
                }

                Op::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err(format!(
                            "trap at ip=0x{:04X}: mod by zero", self.ip
                        ));
                    }
                    self.push(a % b)?;
                }

                Op::Neg => {
                    let a = self.pop()?;
                    self.push(-a)?;
                }

                Op::Load(slot) => {
                    let val = self.globals[slot as usize];
                    self.push(val)?;
                }

                Op::Store(slot) => {
                    let val = self.pop()?;
                    self.globals[slot as usize] = val;
                }

                Op::Print => {
                    let val = self.pop()?;
                    println!("{}", val);
                }

                Op::Halt => {
                    return Ok(());
                }
            }
        }
    }
}