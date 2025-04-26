use std::{fs::File, io::BufReader, path::PathBuf};

use tar::Archive;
use zstd::stream::Decoder;

pub fn unpack_package(filename: &String, destination: &PathBuf, verbose: bool) {
    let file = File::open(filename).unwrap_or_else(|e| {
        panic!(
            "[ERROR] .stm file opening failed: {}\n{}",
            filename,
            e.to_string()
        )
    });
    let mut bufreader = BufReader::new(file);

    depack_archive(&mut bufreader, destination, verbose);
}

fn depack_archive(reader: &mut BufReader<File>, destination: &PathBuf, verbose: bool) {
    write_if(verbose, "Starting decoding stream for the .stm file.");
    let decoder = Decoder::new(reader).expect("Decoder not initialized.");

    write_if(verbose, "Starting unpacking stream for the .stm file.");
    let mut unpacker = Archive::new(decoder);

    write_if(verbose, "Starting depacking...");
    unpacker
        .unpack(destination)
        .unwrap_or_else(|e| panic!("[ERROR] .stm unpacking failed.\n{}", e.to_string()));
}

fn write_if(verbose: bool, message: &str) {
    if verbose {
        println!("[VERBOSE]: {}", message);
    }
}
