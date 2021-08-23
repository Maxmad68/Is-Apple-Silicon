use byteorder::*;
use std::io::{Read, Cursor};
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
    
    let mut rdr = Cursor::new(value);
    let be = rdr.read_u32::<BigEndian>().unwrap();
    
    return be
}

/// Read 4 next bytes of a file and get them as a u32 with little-endian
pub fn read_little_endian(file: &File) -> u32 {
    let value = read_4_bytes(&file);
    
    let mut rdr = Cursor::new(value);
    let be = rdr.read_u32::<LittleEndian>().unwrap();
    
    return be
}