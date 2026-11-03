
#[derive(Debug)]
pub struct ElfHeader {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_phnum: u16,
    pub e_shnum: u16,
}

pub fn parse_elf_header(file_content: &[u8]) -> Result<ElfHeader, &'static str> {

    elf_header_check(file_content)?;

    let header = ElfHeader {
        e_type: read_u16(file_content, 16),
        e_machine: read_u16(file_content, 18),
        e_entry: read_u64(file_content, 24),
        e_phoff: read_u64(file_content, 32),
        e_shoff: read_u64(file_content, 40),
        e_phnum: read_u16(file_content, 56),
        e_shnum: read_u16(file_content, 60),
    };

    Ok(header)
}

pub fn elf_header_check(file_content: &[u8]) -> Result<(), &'static str> {

    if file_content.len() < 64 {
        return Err("Error: file size too short!\n");
    }
    // 7f 45 4c 46
    if file_content[0] != 0x7f ||
        file_content[1] != b'E' || file_content[2] != b'L' ||
        file_content[3] != b'F' {
        return Err("Error: 4 first octets\n");
    }

    if file_content[4] != 0x02 {
        return Err("Error: wrong architecture need 64 bits\n");
    }

    if file_content[5] != 0x01 {
        return Err("Error: need little endian\n");
    }

    if file_content[6] != 0x01 {
        return Err("Error: invalid version\n");
    }
    let e_type: u16 = read_u16(file_content, 16);
    if e_type != 2 && e_type != 3 {
        return Err("Error: invalid binary file\n");
    }

    let e_machine: u16 = read_u16(file_content, 18);
    if e_machine != 62 {
        return Err("Error: invalid machine type\n");
    }

    Ok(())
}

fn read_u16(octet_array: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([octet_array[offset], octet_array[offset + 1]])
}

fn read_u64(octet_array: &[u8], offset: usize) -> u64 {
    let bytes: [u8; 8] = (&octet_array[offset..offset + 8]).try_into().unwrap();

    u64::from_le_bytes(bytes)
}
