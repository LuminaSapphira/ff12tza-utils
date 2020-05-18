use std::io::{Seek, Read, SeekFrom};

const BUFFER_SIZE: usize = 4096;

pub fn locate_signature<R: Read + Seek>(reader: &mut R, signature: &[u8]) -> Option<usize> {
    assert!(signature.len() > 1, "Can't locate an empty signature!");
    assert!(signature.len() < BUFFER_SIZE, "Signature is too long for buffer!");
    let mut buffer = vec![0u8; BUFFER_SIZE].into_boxed_slice();
    let mut matched_end: usize = 0;
    let mut read_bytes: u64 = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(bytes) => {
                if matched_end != 0 {
                    if &buffer[0..signature.len() - matched_end] == &signature[matched_end..] {
                        let pos = reader.seek(SeekFrom::Current(0)).expect("Seeking");
                        return Some(pos as usize - bytes - matched_end);
                    } else {
                        matched_end = 0;
                    }
                }
                if bytes < BUFFER_SIZE {
                    return find_in_slice(&buffer[0..bytes], signature).map(|a| a + read_bytes as usize);
                } else {
                    match find_in_slice(&buffer, signature) {
                        Some(n) => { return Some(n + read_bytes as usize); },
                        None => {
                            'find_partial: for i in 1..signature.len() {
                                let buf_end = &buffer[buffer.len() - i .. buffer.len()];
                                let sig_start = &signature[0..i];
                                if buf_end == sig_start {
                                    matched_end = i
                                }
                            }
                        }
                    }
                }
                read_bytes += bytes as u64;
            },
            Err(err) => {
                eprintln!("Error during I/O. {}", err);
                std::process::exit(-1);
            }
        }
    }
}

fn find_in_slice(slice: &[u8], signature: &[u8]) -> Option<usize> {
    slice
        .windows(signature.len())
        .enumerate()
        .find(|(_, a)| a == &signature)
        .map(|(loc, _)| loc)
}

#[cfg(test)]
mod tests {
    use crate::utils::{locate_signature, find_in_slice};
    use std::io::Cursor;
    use rand::Rng;

    const SIGNATURE: [u8; 8] = [0x65, 0x72, 0x69, 0x69, 0x6E, 0x79, 0x61, 0x61];

    fn fill_random_excl_sig_bytes(data: &mut Vec<u8>) {
        let mut rng = rand::thread_rng();
        for k in data {
            loop {
                *k = rng.gen();
                if !SIGNATURE.contains(k) {
                    break;
                }
            }
        }
    }

    #[test]
    fn test_find_in_slice() {
        let mut data = vec![0u8; 4096];
        let sig = &SIGNATURE[..];
        let insert_slice = &mut data[1234..1234 + sig.len()];
        insert_slice.copy_from_slice(sig);
        assert_eq!(find_in_slice(&data[..], &SIGNATURE[..]).expect("not found"), 1234);
    }

    #[test]
    fn test_locate_sig_easy() {
        let mut data = vec![0u8; 8192];
        let sig = &SIGNATURE[..];
        let insert_slice = &mut data[1234..1234 + sig.len()];
        insert_slice.copy_from_slice(sig);
        let mut cursor = Cursor::new(data);
        assert_eq!(locate_signature(&mut cursor, &SIGNATURE[..]).expect("not found"), 1234);
    }

    #[test]
    fn test_locate_sig_harder() {
        let mut data = vec![0u8; 8192];
        fill_random_excl_sig_bytes(&mut data);
        let sig = &SIGNATURE[..];
        let insert_slice = &mut data[5678..5678 + sig.len()];
        insert_slice.copy_from_slice(sig);
        let mut cursor = Cursor::new(data);
        assert_eq!(locate_signature(&mut cursor, &SIGNATURE[..]).expect("not found"), 5678);
    }

    #[test]
    fn test_locate_sig_border() {
        let mut data = vec![0u8; 8192];
        fill_random_excl_sig_bytes(&mut data);
        let sig = &SIGNATURE[..];
        let insert_slice = &mut data[4092..4092 + sig.len()];
        insert_slice.copy_from_slice(sig);
        let mut cursor = Cursor::new(data);
        assert_eq!(locate_signature(&mut cursor, &SIGNATURE[..]).expect("not found"), 4092);
    }

}
