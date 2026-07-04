mod isa;
mod vm;
mod asm;
mod dis;

use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    
    if args.len() < 3 {
        eprintln!("usage:");
        eprintln!("  minivm asm <file.tasm> -o <file.tbc>");
        eprintln!("  minivm run <file.tbc>");
        eprintln!("  minivm run --trace <file.tbc>");
        eprintln!("  minivm dis <file.tbc>");
        std::process::exit(1);
    }


    match args[1].as_str() {

        
        "asm" => {
        
            let input_file = &args[2];

    
            let output_file = if args.len() >= 5 && args[3] == "-o" {
                args[4].clone()
            } else {
                input_file.replace(".tasm", ".tbc")
            };

            
            let text = fs::read_to_string(input_file).unwrap_or_else(|e| {
                eprintln!("could not read {}: {}", input_file, e);
                std::process::exit(1);
            });

            
            match asm::assemble(&text) {
                Ok(bytecode) => {
            
                    fs::write(&output_file, bytecode).unwrap_or_else(|e| {
                        eprintln!("could not write {}: {}", output_file, e);
                        std::process::exit(1);
                    });
                    println!("assembled successfully → {}", output_file);
                }
                Err(e) => {
                    
                    eprintln!("assembly error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        
        "run" => {
            
            let trace = args.contains(&"--trace".to_string());

            
            let input_file = if trace && args.len() >= 4 {
                &args[3]
            } else {
                &args[2]
            };

        
            let bytes = fs::read(input_file).unwrap_or_else(|e| {
                eprintln!("could not read {}: {}", input_file, e);
                std::process::exit(1);
            });

            
            let code = match parse_header(&bytes) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("invalid bytecode file: {}", e);
                    std::process::exit(1);
                }
            };

        
            let mut machine = vm::Vm::new();
            let result = machine.run(code, trace);

            if let Err(e) = result {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        
        "dis" => {
            let input_file = &args[2];
            let bytes = fs::read(input_file).unwrap_or_else(|e| {
                eprintln!("could not read {}: {}", input_file, e);
                std::process::exit(1);
            });

    
            let code = match parse_header(&bytes) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("invalid bytecode file: {}", e);
                    std::process::exit(1);
                }
            };

        
            println!("{}", dis::disassemble(code));
        }

        
        unknown => {
            eprintln!("unknown command '{}' — use asm, run, or dis", unknown);
            std::process::exit(1);
        }
    }
}


fn parse_header(bytes: &[u8]) -> Result<&[u8], String> {
    
    if bytes.len() < 9 {
        return Err("file is too short to be valid".to_string());
    }

    
    if &bytes[0..4] != &[0x4D, 0x56, 0x4D, 0x00] {
        return Err("magic bytes are wrong — not a minivm file".to_string());
    }

    
    if bytes[4] != 0x01 {
        return Err("unsupported version number".to_string());
    }

    
    let code_length = u32::from_le_bytes(
        bytes[5..9].try_into().unwrap()
    ) as usize;

    
    if bytes.len() < 9 + code_length {
        return Err("file is truncated — code is shorter than header says".to_string());
    }

    
    Ok(&bytes[9..9 + code_length])
}
