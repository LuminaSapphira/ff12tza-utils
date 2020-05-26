use std::path::PathBuf;
use crate::assert_exists;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader, BufRead};
use byteorder::{ReadBytesExt, LE};
use serde::Serialize;
use std::ffi::CStr;

const MAGIC: u32 = 0x4b595253;

pub fn analyze(vbf: PathBuf) {
    assert_exists!(vbf, "vbf");
    let mut file = BufReader::new(File::open(&vbf).expect("VBF open"));
    let magic = file.read_u32::<LE>().expect("reading");
    assert_eq!(magic, MAGIC, "VBF Magic incorrect.");
    let header_len = file.read_u32::<LE>().expect("reading");
    let num_files = file.read_u64::<LE>().expect("reading");
    println!("Header length: {} (0x{:x})", header_len, header_len);
    println!("Num Files: {}", num_files);

    let mut md5_vec = Vec::with_capacity(num_files as usize);
    let mut file_vec = Vec::with_capacity(num_files as usize);

    for _ in 0..num_files {
        let mut md5 = [0u8; 16];
        file.read_exact(&mut md5).expect("reading");
        md5_vec.push(md5);
    }
    let mut drain = md5_vec.drain(..);
    for _ in 0..num_files {
        file_vec.push(VbfFile {
            block_list_start: file.read_u32::<LE>().expect("reading"),
            original_size: {
                file.read_u32::<LE>().expect("reading");
                file.read_u64::<LE>().expect("reading")
            },
            start_offset: file.read_u64::<LE>().expect("reading"),
            name: {
                let offset = file.read_u64::<LE>().expect("reading");
                let name_offset = 0x14 + num_files * 48 + offset;
                let last_pos = file.seek(SeekFrom::Current(0)).expect("seeking");
                file.seek(SeekFrom::Start(name_offset)).expect("seeking");
                let mut name_bytes = Vec::new();
                file.read_until(0, &mut name_bytes).expect("reading");
                file.seek(SeekFrom::Start(last_pos)).expect("seeking");
                let name_cstr = CStr::from_bytes_with_nul(&name_bytes).expect("reading cstr");
                let name = name_cstr.to_str().expect("UTF-8");
                String::from(name)
            },
            md5: drain.next().unwrap()
        });
    }

    // let webm = file_vec.get(0).expect("getting file");
    // let offset = webm.start_offset;
    // let size = webm.original_size;
    // let mut data = Vec::with_capacity(size as usize);
    // let mut data = unsafe {
    //     data.set_len(size as usize);
    //     data.into_boxed_slice()
    // };
    // file.seek(SeekFrom::Start(offset)).expect("seeking");
    // file.read_exact(&mut data);
    // File::create("output.webm").expect("owo").write_all(&data).expect("uwu");


    serde_json::to_writer_pretty(File::create("vbf_analysis.json").expect("creating output"), &file_vec).expect("writing json");

    // let mut str_table_len = file.read_u32::<LE>().expect("reading");
    // file.read

    println!("Debugging. Vbf exists.")
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize)]
struct VbfFile {
    block_list_start: u32,
    original_size: u64,
    md5: [u8; 16],
    start_offset: u64,
    name: String
}
