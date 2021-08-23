use colour::*;

use std::io::{Seek, SeekFrom};

use std::env;
use std::fs::File;
use std::path::Path;
use std::process::exit;

mod read_bin;


// Magic numbers
const MAGIC_FAT: u32 =               0xCAFEBABE;
const MAGIC_BIG_ENDIAN: u32 =        0xFEEDFACE;
const MAGIC_64_BIG_ENDIAN: u32 =     0xFEEDFACF;
const MAGIC_LITTLE_ENDIAN: u32 =     0xCEFAEDFE;
const MAGIC_64_LITTLE_ENDIAN: u32 =  0xCFFAEDFE;


// Architectures
const CPU_I386: u32 =	0x00000007;
const CPU_X86_64: u32 =	0x01000007;
const CPU_ARM: u32 =		0x0000000C;
const CPU_ARM64: u32 =	0x0100000C;
const CPU_PPC: u32 =		0x00000012;
const CPU_PPC64: u32 =	0x01000012;



/// Get architecture name from CPU number
fn arch_cpu_name(cpu: u32) -> String {
	match cpu {
		CPU_I386 => "i386",
		CPU_X86_64 => "x86_64",
		CPU_ARM => "arm",
		CPU_ARM64 => "arm64",
		CPU_PPC => "ppc",
		CPU_PPC64 => "ppc64",
		_ => "unknown"
	}.to_string()
}


/// Get list of architectures for binary path
fn find_architectures(binary_path: String) -> Vec<u32> {
	
	let mut archs = Vec::new();
	
	let mut file = File::open(binary_path).expect("File IO Failure");
	
	
	let magic = read_bin::read_big_endian(&file);
	
	if magic == MAGIC_FAT { // Fat binary
		
		let count = read_bin::read_big_endian(&file); // How many architectures in the archive?
		
		for _ in 0..count { 
			let arch = read_bin::read_big_endian(&file);
			let _ = file.seek( // Skip the additionnal informations for this arch
				SeekFrom::Current(16)
			);
			
			archs.push(arch);
		}
		
		
		
	} else if magic == MAGIC_BIG_ENDIAN ||
				magic == MAGIC_64_BIG_ENDIAN { // One architecture, Big-Endian
					
		let arch = read_bin::read_big_endian(&file);
		archs.push(arch);
		
		
		
	} else if magic == MAGIC_LITTLE_ENDIAN ||
				magic == MAGIC_64_LITTLE_ENDIAN { // One architecture, Little-Endian
		
		let arch = read_bin::read_little_endian(&file);
		archs.push(arch);
		
		
	} else {
		eprintln!("Not a valid MachO file");
		exit(2);
	}
	
	return archs;
	
}


/// Main program, for one given binary path.
fn list_architectures_ui(binary_path: String) {
	
	let archs = find_architectures(binary_path);
	let mut is_arm64 = false;
	
	println!("Supported architectures:");
	
	for arch in archs {
		let name = arch_cpu_name(arch);
		println!(" - {}", name);
		
		is_arm64 |= arch == CPU_ARM64;
	}
	
	
	if is_arm64 {
		green_ln!("Executable is Apple Silicon compatible");
	} else {
		red_ln!("Executable is not Apple Silicon compatible");
	}
}


/// Get the path of the given program name.
/// If the parameter is already a binary path, just returns it.
/// Otherwise, look into PATH environment variable to determine
/// the program location.
fn binary(name: &str) -> String {
	if Path::new(name).exists() {
		return name.to_string();
	} else {
		let path = env!("PATH")
			.split(":")
			.map( |path|
				if path.ends_with("/") {
					path.to_owned() + name
				} else {
					path.to_owned() + "/" + name
				})
			.filter(|path|
					Path::new(path).exists())
			.next();
		
		if let Some(s) = path {
			return s;
		} else {
			eprintln!("Can't find program \"{}\"", name);
			exit(1);
		}

	}
}



fn main() {	
	
	for argv in env::args().skip(1) {
		
		let path = binary(&argv.to_string());
		
		println!("{}:", path);
		list_architectures_ui(path);
		println!();
	}
	
	exit(0);
}
	