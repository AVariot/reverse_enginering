
mod elf_handler;
mod options;

use std::env;
use std::fs;
use std::process::exit;

use crate::elf_handler::ProgramHeader;

#[derive(Debug)]
struct ArgChest {
    h: bool,
    path: Vec<String>
}

fn file_handler(file_path: &str) -> Vec<u8> {
    let content = match fs::read(file_path) {
        Ok(octets) => octets,
        Err(error) => {
            eprintln!("Lecture impossible : {}\n", error);
            exit(84);
        }
    };

    content
}

fn parse_arg(args_array: Vec<String>) -> Result<ArgChest, String> {
    let mut args_storage: ArgChest = ArgChest { h: false, path: Vec::new() };

    for arg in args_array {
        match arg.as_str() {
            "-h" => args_storage.h = true,
            _ => {
                if arg.starts_with('-') {
                    return Err(format!("Error: argument {} unknown", arg));
                }
                args_storage.path.push(arg);
            }
        }
    }

    if args_storage.path.is_empty() {
        return Err("Error: binary path is empty".to_string());
    }

    Ok(args_storage)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        eprintln!("Error: args size error!");
        exit(84)
    }
    let args_chest: ArgChest = match parse_arg(args) {
        Ok(chest) => chest,
        Err(message) => {
            eprintln!("{}", message.to_string());
            exit(84);
        }
    };

    for binary in args_chest.path {
        let file_content: Vec<u8> = file_handler(&binary);
        let elf_header = match elf_handler::parse_elf_header(&file_content) {
            Ok(header) => header,
            Err(message) => {
                eprintln!("{}\n", message.to_string());
                exit(84);
            }
        };
    
        let program_header: Vec<ProgramHeader> = match elf_handler::parse_programe_headers(&file_content, &elf_header) {
            Ok(headers) => headers,
            Err(message) => {
                eprintln!("{}\n", message.to_string());
                exit(84);
            }
        };

        if args_chest.h {
            options::h::option_h(&file_content[0..16], elf_header);
        }
    }

    println!("Ok");
    exit(0);
}
