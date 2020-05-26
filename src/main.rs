mod error;
mod opt;
mod treasure;
mod magick_order;
mod battle_pack;
mod utils;
mod vbf;

use opt::Opts;
use structopt::StructOpt;
use std::io::Write;

#[macro_export]
macro_rules! assert_exists {
    ($file:expr, $desc:expr) => {
        if !$file.exists() { eprintln!("Missing {} file", $desc); std::process::exit(1); }
    };
}

#[macro_export]
macro_rules! error_abort {
    ($code:expr) => { error_exit($code, format_args!()); };
    ($code:expr, $($arg:tt)*) => { $crate::error_exit($code, format_args!($($arg)*)); };
}

#[inline]
fn error_exit(code: i32, args: std::fmt::Arguments) -> ! {
    std::io::stderr().write_fmt(args).expect("Writing to stderr");
    std::io::stderr().write(&['\n' as u8]).expect("Writing to stderr");
    std::process::exit(code);
}

fn main() {
    let opts: Opts = Opts::from_args();
    match opts {
        Opts::DumpTreasure { input, output, treasure_data, item_data } => treasure::dump_treasure(input, output, treasure_data, item_data),
        Opts::ReorderMagick { battle_pack, magick_order, output } => magick_order::reorder_magick(battle_pack, magick_order, output),
        Opts::BattlePack(bp) => match_battle_pack(bp),
        Opts::VBF(vbf) => match_vbf(vbf),
    }
}

fn match_vbf(opts: opt::Vbf) {
    match opts {
        opt::Vbf::Analyze { vbf } => vbf::analyze(vbf),
    }
}

#[cfg(feature = "battle_fuse")]
#[allow(unused)]
fn match_battle_pack(opts: opt::BattlePack) {
    match opts {
        opt::BattlePack::Unpack {battle_pack, output} => { battle_pack::unpack(battle_pack, output); },
        opt::BattlePack::Repack {input, battle_pack} => {},
        opt::BattlePack::AllowAllFlying {battle_pack} => {},
        opt::BattlePack::Fuse { battle_pack, mount_point } => {}
    }
}

#[cfg(not(feature = "battle_fuse"))]
#[allow(unused)]
fn match_battle_pack(opts: opt::BattlePack) {
    match opts {
        opt::BattlePack::Unpack {battle_pack, output} => { battle_pack::unpack(battle_pack, output); },
        opt::BattlePack::Repack {input, battle_pack} => {},
        opt::BattlePack::AllowAllFlying {battle_pack} => battle_pack::allow_all_flying(battle_pack),
        #[allow(unreachable_patterns)]
        _ => unreachable!()
    }
}


