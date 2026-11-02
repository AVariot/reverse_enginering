
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
    let e_type: u16 = u16::from_le_bytes([file_content[16], file_content[17]]);
    if e_type != 2 && e_type != 3 {
        return Err("Error: invalid binary file\n");
    }

    if file_content[18] != 0x3e ||
        file_content[19] != 0x00 {
        return Err("Error: invalid machine type\n");
    }

    Ok(())
}
