use crate::isa::{decode, Op};

pub fn disassemble(code: &[u8]) -> String {
    
    let mut lines: Vec<String> = Vec::new();

    
    let mut pos = 0;

    
    while pos < code.len() {

    
        let (op, size) = decode(code, pos);

    
        let text = match op {
            Op::Push(n)    => format!("PUSH {}", n),
            Op::Pop        => "POP".to_string(),
            Op::Dup        => "DUP".to_string(),
            Op::Swap       => "SWAP".to_string(),
            Op::Add        => "ADD".to_string(),
            Op::Sub        => "SUB".to_string(),
            Op::Mul        => "MUL".to_string(),
            Op::Div        => "DIV".to_string(),
            Op::Mod        => "MOD".to_string(),
            Op::Neg        => "NEG".to_string(),
            Op::Load(slot) => format!("LOAD {}", slot),
            Op::Store(slot)=> format!("STORE {}", slot),
            Op::Print      => "PRINT".to_string(),
            Op::Halt       => "HALT".to_string(),
        };

        
        lines.push(text);

        
        pos += size;
    }

    
    lines.join("\n")
}