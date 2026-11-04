use crate::elf_handler;


pub fn option_h(magic: &[u8], elf_header: elf_handler::ElfHeader) {
    println!("ELF Header:");
    print!("  Magic:   ");
    for octet in magic {
        print!("{:02x} ", octet);
    }
    println!();
    println!("  {:<35}{}", "Class:", "ELF64");
    println!("  {:<35}{}", "Data:", "2's complement, little endian");
    println!("  {:<35}{}", "Version:", "1 (current)");
    println!("  {:<35}{}", "OS/ABI:", elf_handler::os_name(elf_header.EI_OSABI));
    println!("  {:<35}{}", "ABI Version:", elf_header.EI_ABIVERSION);
    println!("  {:<35}{}", "Type:", elf_handler::type_string(elf_header.e_type));
    println!("  {:<35}{}", "Machine:", "Advanced Micro Devices X86-64");
    println!("  {:<35}{:#x}", "Version:", elf_header.e_version);
    println!("  {:<35}{:#x}", "Entry point address:", elf_header.e_entry);
    println!("  {:<35}{} (bytes into file)", "Start of program headers:", elf_header.e_phoff);
    println!("  {:<35}{} (bytes into file)", "Start of section headers:", elf_header.e_shoff);
    println!("  {:<35}{:#x}", "Flags:", elf_header.e_flags);
    println!("  {:<35}{} (bytes)", "Size of this header:", elf_header.e_ehsize);
    println!("  {:<35}{} (bytes)", "Size of program headers:", elf_header.e_phentsize);
    println!("  {:<35}{}", "Number of program headers:", elf_header.e_phnum);
    println!("  {:<35}{} (bytes)", "Size of section headers:", elf_header.e_shentsize);
    println!("  {:<35}{}", "Number of section headers:", elf_header.e_shnum);
    println!("  {:<35}{}", "Section header string table index:", elf_header.e_shstrndx);
}
