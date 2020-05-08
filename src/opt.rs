
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(about = "Utilities for FFXII: TZA modding")]
pub enum Opts {
    DumpTreasure {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
        #[structopt(parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(short, long, parse(from_os_str), env, default_value = "data/treasure_data.json")]
        treasure_data: PathBuf,
        #[structopt(short, long, parse(from_os_str), env, default_value = "data/item_data.json")]
        item_data: PathBuf,
    },
    ReorderMagick {
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
        #[structopt(parse(from_os_str))]
        magick_order: PathBuf,
    },
}
