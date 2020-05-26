use std::path::PathBuf;

#[cfg(feature = "battle_fuse")]
mod fuse;

mod reader;

use crate::{assert_exists, error_abort};
use crate::utils;
use std::fs::{File, OpenOptions, DirBuilder};
use std::io::{Seek, SeekFrom, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

use reader::BattlePackReader;

const EQUIPMENT_SIGNATURE: [u8; 3] = [68, 113, 0];
const OFFSET_FROM_SIGNATURE: usize = 8;
const FLYING_FLAG_OFFSET: usize = 7;
const EQUIPMENT_STRUCT_SIZE: usize = 52;

pub fn unpack(battle_pack: PathBuf, output: Option<PathBuf>) {
    assert_exists!(battle_pack, "battle pack");
    let output = output.unwrap_or_else(|| battle_pack.with_extension("unpacked"));

    if let Err(err) = DirBuilder::new().recursive(true).create(output.as_path()) {
        error_abort!(1, "Failed to create output folder. Error: {}", err);
    }

    let bp_file = match File::open(&battle_pack) {
        Ok(file) => file,
        Err(err) => {
            error_abort!(1, "Failed to open battle pack '{:?}' for reading. Error: {}", &battle_pack, err)
        }
    };

    let mut bp_reader = match BattlePackReader::new(bp_file) {
        Ok(reader) => reader,
        Err(err) => {
            error_abort!(2, "Failed to create reader over battle pack. Error: {}", err)
        }
    };

    for i in 0..bp_reader.section_count() {
        let mut output_bin = {
            let out_file_path = output.join(format!("section_{:02}.bin", i));
            let output_path = out_file_path.as_path();
            match File::create(output_path) {
                Ok(file) => file,
                Err(err) => {
                    error_abort!(3, "Failed to create output file '{:?}'. Error: {}", output_path, err);
                }
            }
        };
        let mut buffer = Vec::new();
        // match bp_reader.section_size(i) {
        match bp_reader.section_begin_to_end(i, &mut buffer) {
            Ok(d) => {
                println!("Exporting section {}, {} bytes.", i, d);
                if let Err(err) = output_bin.write_all(&buffer) {
                    error_abort!(4, "Failed to write export for section {}. Error: {}", i, err);
                }
                buffer.clear();
            },
            Err(err) => {
                error_abort!(2, "Failed to read data for section {}. Error: {}", i, err);
            }
        }
    }

}

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
