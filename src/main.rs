
mod elf_handler;

use std::env;
use std::fs;
use std::process::exit;

use crate::elf_handler::ProgramHeader;

fn file_handler(file_path: &str) -> Vec<u8> {
    let content = match fs::read(file_path) {
        Ok(octets) => octets,
        Err(error) => {
            eprintln!("Lecture impossible : {}", error);
            exit(84);
        }
    };

    content
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Error: args size error!");
        exit(84)
    }

    let file_content: Vec<u8> = file_handler(&args[1]);
    let elf_header = match elf_handler::parse_elf_header(&file_content) {
        Ok(header) => header,
        Err(message) => {
            eprint!("{}", message.to_string());
            exit(84);
        }
    };

    let program_header: Vec<ProgramHeader> = match elf_handler::parse_programe_headers(&file_content, &elf_header) {
        Ok(headers) => headers,
        Err(message) => {
            eprint!("{}", message.to_string());
            exit(84);
        }
    };

    println!("{:#?}", elf_header);
    println!("{:#?}", program_header);

    println!("Ok");
    exit(0);
}
