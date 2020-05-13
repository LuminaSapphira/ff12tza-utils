mod error;
mod opt;
mod treasure;
mod magick_order;
mod battle_pack;

use opt::Opts;
use structopt::StructOpt;

use desert_auto::desert;

#[desert(owo)]
fn main() {
    let opts: Opts = Opts::from_args();
    match opts {
        Opts::DumpTreasure { input, output, treasure_data, item_data } => treasure::dump_treasure(input, output, treasure_data, item_data),
        Opts::ReorderMagick {battle_pack, magick_order, output} => magick_order::reorder_magick(battle_pack, magick_order, output),
        _ => {}
    }
}

#[cfg(feature = "battle_fuse")]
fn match_battle_pack(opts: opt::BattlePack) {
    match opts {
        opt::BattlePack::Unpack {battle_pack, output} => {},
        opt::BattlePack::Repack {input, battle_pack} => {},
        opt::BattlePack::Fuse {battle_pack, mount_point} => {},
    }
}

#[cfg(not(feature = "battle_fuse"))]
fn match_battle_pack(opts: opt::BattlePack) {
    match opts {
        opt::BattlePack::Unpack {battle_pack, output} => {},
        opt::BattlePack::Repack {input, battle_pack} => {},
        _ => unreachable!()
    }
}


