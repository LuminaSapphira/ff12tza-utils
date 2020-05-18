use std::path::PathBuf;

#[cfg(feature = "battle_fuse")]
mod fuse;

use crate::assert_exists;
use crate::utils;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use byteorder::{ReadBytesExt, WriteBytesExt};

const EQUIPMENT_SIGNATURE: [u8; 3] = [68, 113, 0];
const OFFSET_FROM_SIGNATURE: usize = 8;
const FLYING_FLAG_OFFSET: usize = 7;
const EQUIPMENT_STRUCT_SIZE: usize = 52;

pub fn allow_all_flying(battle_pack: PathBuf) {
    assert_exists!(battle_pack, "battle pack");
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    let mut file = match options.open(&battle_pack) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Unable to open file: {:?}\nError: {}", &battle_pack, err);
            std::process::exit(-1);
        }
    };
    let equip_array = match utils::locate_signature(&mut file, &EQUIPMENT_SIGNATURE[..]) {
        Some(loc) => loc + OFFSET_FROM_SIGNATURE,
        None => {
            eprintln!("Unable to find the equipment section within the battle pack.");
            std::process::exit(7);
        }
    };
    println!("Located appropriate section.");
    for id in (0usize..=199).map(|a| a * EQUIPMENT_STRUCT_SIZE + equip_array + FLYING_FLAG_OFFSET) {
        file.seek(SeekFrom::Start(id as u64)).expect("Seeking file");
        let byte = file.read_u8().expect("Reading file");
        file.seek(SeekFrom::Start(id as u64)).expect("Seeking file");
        file.write_u8(byte | 0b100).expect("Writing file");
    }

    println!("Made all weapons in battle pack able to hit flying enemies.");

}
