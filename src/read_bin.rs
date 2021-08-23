use std::io::Read;
use std::fs::File;

/// Read 4 next bytes of a file into an array of 4 u8
pub fn read_4_bytes(mut file: &File) -> [u8; 4] {
    let mut value = [0u8; 4];
    file.read_exact(&mut value).expect("Unable to read 4 bytes");
    return value;
}

/// Read 4 next bytes of a file and get them as a u32 with big-endian
pub fn read_big_endian(file: &File) -> u32 {
    let value = read_4_bytes(&file);
    
    let n1: u32 = value[0].into();
    let n2: u32 = value[1].into();
    let n3: u32 = value[2].into();
    let n4: u32 = value[3].into();
    
    (n1 << 24) | (n2 << 16) | (n3 << 8) | n4
    
}

/// Read 4 next bytes of a file and get them as a u32 with little-endian
pub fn read_little_endian(file: &File) -> u32 {
    let value = read_4_bytes(&file);
    
    let n1: u32 = value[0].into();
    let n2: u32 = value[1].into();
    let n3: u32 = value[2].into();
    let n4: u32 = value[3].into();
    
    (n4 << 24) | (n3 << 16) | (n2 << 8) | n1
}