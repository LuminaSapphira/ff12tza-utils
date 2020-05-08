use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

mod base_order;

macro_rules! assert_exists {
    ($file:expr, $desc:expr) => {
        if !$file.exists() { eprintln!("Missing {} file", $desc); std::process::exit(1); }
    };
}

fn open_file(pb: PathBuf) -> File {
    match File::open(&pb) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Unable to open file {:?}. Error: {}", &pb, err);
            std::process::exit(2);
        }
    }
}

fn read_requested_order(file: File) -> Vec<String> {
    match serde_json::from_reader(file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to read or parse magick order file: {}", e);
            std::process::exit(3);
        }
    }
}

fn calculate_file_order(requested_order: Vec<String>) -> Vec<u8> {
    let order = base_order::base_order();

    let mut file_order = Vec::with_capacity(order.order.len());
    file_order.resize(order.order.len(), 0u8);
    for (i, spell) in requested_order.into_iter().enumerate() {
        let game_index = order.map[&spell.as_str()];
        file_order[game_index] = i as u8;
    }
    file_order
}

fn read_to_end(file: File) -> Vec<u8> {
    let mut file = file;
    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => data,
        Err(err) => {
            eprintln!("Unable to read file. Error: {}", err);
            std::process::exit(4);
        }
    }
}

const MAGICK_SIGNATURE: [u8; 12] = [81, 0, 0, 0, 8, 0, 0, 0, 32, 0, 0, 0];

pub fn reorder_magick(battle_pack: PathBuf, magick_order: PathBuf, output: PathBuf) {
    assert_exists!(battle_pack, "battle pack");
    assert_exists!(magick_order, "magick order data");
    let file_order = calculate_file_order(read_requested_order(open_file(magick_order)));

    let mut battle_pack_bin = read_to_end(open_file(battle_pack));


    let base_offset = match battle_pack_bin.windows(MAGICK_SIGNATURE.len()).enumerate().find(|(_, a)| a == &MAGICK_SIGNATURE) {
        Some((i, _)) => i + 28,
        None => {
            eprintln!("Unable to find magick signature in battle pack.");
            std::process::exit(5);
        }
    };

    for (i, sort) in file_order.into_iter().enumerate() {
        let offset = base_offset + 8 * i + 6;
        battle_pack_bin[offset] = sort;
    }

    match File::create(&output)
        .and_then(|mut file| file.write_all(&battle_pack_bin)) {
        Ok(_) => {
            println!("Successfully updated battle pack binary and wrote result to {:?}", &output);
        },
        Err(err) => {
            eprintln!("Failed to write output data. Error: {}", err);
        }
    }

}

