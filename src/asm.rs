use crate::isa::{encode, Op};

pub fn assemble(text: &str) -> Result<Vec<u8>, String> {
    
    let mut code: Vec<u8> = Vec::new();

    
    for (line_num, line) in text.lines().enumerate() {

        
        let line = match line.split(';').next() {
            Some(l) => l.trim(),
            None => continue,
        };

        
        if line.is_empty() {
            continue;
        }

        
        let parts: Vec<&str> = line.split_whitespace().collect();

        
        let op = match parts[0].to_uppercase().as_str() {

            
            "PUSH" => {
                
                if parts.len() < 2 {
                    return Err(format!("line {}: PUSH needs a number", line_num + 1));
                }
                let n: i64 = parts[1].parse().map_err(|_| {
                    format!("line {}: '{}' is not a valid number", line_num + 1, parts[1])
                })?;
                Op::Push(n)
            }

            
            "POP"  => Op::Pop,
            "DUP"  => Op::Dup,
            "SWAP" => Op::Swap,
            "ADD"  => Op::Add,
            "SUB"  => Op::Sub,
            "MUL"  => Op::Mul,
            "DIV"  => Op::Div,
            "MOD"  => Op::Mod,
            "NEG"  => Op::Neg,

            
            "LOAD" => {
                if parts.len() < 2 {
                    return Err(format!("line {}: LOAD needs a slot number", line_num + 1));
                }
                let slot: u8 = parts[1].parse().map_err(|_| {
                    format!("line {}: '{}' is not a valid slot number", line_num + 1, parts[1])
                })?;
                Op::Load(slot)
            }

            
            "STORE" => {
                if parts.len() < 2 {
                    return Err(format!("line {}: STORE needs a slot number", line_num + 1));
                }
                let slot: u8 = parts[1].parse().map_err(|_| {
                    format!("line {}: '{}' is not a valid slot number", line_num + 1, parts[1])
                })?;
                Op::Store(slot)
            }

            "PRINT" => Op::Print,
            "HALT"  => Op::Halt,

            
            other => {
                return Err(format!("line {}: don't know what '{}' means", line_num + 1, other));
            }
        };

        
        code.extend_from_slice(&encode(op));
    }

    Ok(add_header(&code))
}

fn add_header(code: &[u8]) -> Vec<u8> {
    
    let mut file: Vec<u8> = Vec::new();

    
    
    file.extend_from_slice(&[0x4D, 0x56, 0x4D, 0x00]);

    
    file.push(0x01);

    let code_length = code.len() as u32;
    file.extend_from_slice(&code_length.to_le_bytes());

    file.extend_from_slice(code);

    file
}

