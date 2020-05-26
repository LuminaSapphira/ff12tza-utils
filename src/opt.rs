
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(about = "Utilities for FFXII: TZA modding")]
pub enum Opts {
    /// Dump all treasure info (position, contents, chances, etc.)
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
    /// Reorder the magick sort list in the battle pack
    ReorderMagick {
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
        #[structopt(parse(from_os_str))]
        magick_order: PathBuf,
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
    /// Utilities for unpacking the battle pack
    BattlePack(BattlePack),
    /// Utilities regarding the .VBF file
    VBF(Vbf),
}
#[derive(StructOpt, Debug)]
pub enum Vbf {
    /// Analyze the provided VBF
    Analyze {
        #[structopt(parse(from_os_str))]
        vbf: PathBuf,
    }
}

#[derive(StructOpt, Debug)]
pub enum BattlePack {
    /// Unpack the BattlePack to a directory of JSON files
    Unpack {
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
        #[structopt(parse(from_os_str))]
        output: PathBuf
    },
    /// Repack the directory created by unpack into a battle_pack.bin
    Repack {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
    },
    /// Modify the provided battle pack to allow all weapons to hit flying enemies
    AllowAllFlying {
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
    },
    #[cfg(feature = "battle_fuse")]
    /// Create a FUSE of the battle_pack, in the same format as unpack
    Fuse {
        #[structopt(parse(from_os_str))]
        battle_pack: PathBuf,
        #[structopt(parse(from_os_str))]
        mount_point: PathBuf
    }
}