
mod elf_handler;

use std::env;
use std::fs;
use std::process::exit;

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
    let header = match elf_handler::parse_elf_header(&file_content) {
        Ok(header) => header,
        Err(message) => {
            eprint!("{}", message.to_string());
            exit(84);
        }
    };

    println!("{:#?}", header);

    println!("Ok");
    exit(0);
}
