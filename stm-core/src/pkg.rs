use std::io::{Cursor, Read, Seek};

pub fn verify_tar_zstd_file(buffer: &Vec<u8>) -> bool {
    if buffer.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
        // File is a zstandard compressed file.
        // Next part to peel another layer off the onion.

        let mut decompressed = vec![];
        decompress_the_file(buffer, &mut decompressed);

        return check_if_tar_file(&decompressed);
    }
    false
}

/*
pub fn verify_stm_file(filename: &str) -> bool {
    let mut file = File::open(filename);
    if let Ok(f) = &mut file {
        let mut file_header = vec![];

        // The
        f.read_to_end(&mut file_header)
            .expect("'read_exact' threw an unknown error.");

        if file_header.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
            // This means this is a file compressed with Zstandard.
            // But we need to make sure if this is also a tar file.
            //
            let mut output = vec![];
            decode_the_file(&file_header, &mut output);

            let mut cursing = Cursor::new(&output);
            cursing.seek(std::io::SeekFrom::Current(257)).unwrap();

            // std::fs::write("output.dmp", &cursing).unwrap();

            println!("output: {:x?}", cursing);

            return true;
        }
        return false;
    }

    false
}
*/

fn check_if_tar_file(buffer: &Vec<u8>) -> bool {
    let mut cursor = Cursor::new(buffer.as_slice());
    cursor
        .seek(std::io::SeekFrom::Start(257))
        .expect("Seek was unsuccessful");

    let mut read_from = [0; 8];
    cursor.read(&mut read_from).unwrap();
    if read_from.eq(&[0x75, 0x73, 0x74, 0x61, 0x72, 0x20, 0x20, 0x00])
        || read_from.eq(&[0x75, 0x73, 0x74, 0x61, 0x72, 0x00, 0x30, 0x30])
    {
        // This is a TAR header.
        return true;
    }

    false
}

fn decompress_the_file(encoded: &[u8], decoded: &mut Vec<u8>) {
    let mut decoder =
        zstd::stream::Decoder::new(encoded).expect("'Decoder' threw an unknown error.");

    decoder.read_to_end(decoded).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_should_be_tarzst() {
        let mut file = File::open("test/should_be_a.tar.zst").unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();

        let result = verify_tar_zstd_file(&buffer);
        assert_eq!(result, true);
    }

    #[test]
    fn test_should_not_verify_zst_file() {
        let mut file = File::open("test/imposter.zst").unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();

        let result = verify_tar_zstd_file(&buffer);
        assert_eq!(result, false);
    }
}
