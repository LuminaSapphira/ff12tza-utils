mod error;
mod opt;
mod treasure;
mod magick_order;

use opt::Opts;
use structopt::StructOpt;

fn main() {
    let opts: Opts = Opts::from_args();
    match opts {
        Opts::DumpTreasure { input, output, treasure_data, item_data } => treasure::dump_treasure(input, output, treasure_data, item_data),
        Opts::ReorderMagick {battle_pack, magick_order} => magick_order::reorder_magick(battle_pack, magick_order)
    }
}


