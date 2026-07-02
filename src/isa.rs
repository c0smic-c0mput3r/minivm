#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64),
    Pop, 
    Dup, 
    Swap,
    Add, 
    Sub, 
    Mul, 
    Div, 
    Mod, 
    Neg,
    Load(u8), 
    Store(u8),
    Print, 
    Halt,
}
pub fn encode(op: Op) -> Vec<u8>{
    match op{
        Op::Push(n)=>{
            let mut v=vec![0x01];
            v.extend_from_slice(&n.to_le_bytes());
            v
        }
        Op::Pop      => vec![0x02],
        Op::Dup      => vec![0x03],
        Op::Swap     => vec![0x04],
        Op::Add      => vec![0x10],
        Op::Sub      => vec![0x11],
        Op::Mul      => vec![0x12],
        Op::Div      => vec![0x13],
        Op::Mod      => vec![0x14],
        Op::Neg      => vec![0x15],
        Op::Load(s)  => vec![0x40, s],
        Op::Store(s) => vec![0x41, s],
        Op::Print    => vec![0x60],
        Op::Halt     => vec![0xFF],
    }
}
pub fn decode(bytes: &[u8], pos: usize) -> (Op, usize) {
    match bytes[pos] {
        0x01 => {
            let arr: [u8; 8] = bytes[pos+1..pos+9].try_into().unwrap();
            let n = i64::from_le_bytes(arr);
            (Op::Push(n), 9)
        }
        0x02 => (Op::Pop,            1),
        0x03 => (Op::Dup,            1),
        0x04 => (Op::Swap,           1),
        0x10 => (Op::Add,            1),
        0x11 => (Op::Sub,            1),
        0x12 => (Op::Mul,            1),
        0x13 => (Op::Div,            1),
        0x14 => (Op::Mod,            1),
        0x15 => (Op::Neg,            1),
        0x40 => (Op::Load(bytes[pos+1]),  2),
        0x41 => (Op::Store(bytes[pos+1]), 2),
        0x60 => (Op::Print,          1),
        0xFF => (Op::Halt,           1),
        b    => panic!("unknown opcode 0x{:02X} at pos {}", b, pos),
    }
}
