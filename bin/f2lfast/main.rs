mod patterns;

mod search;

mod triggers;

use std::{process::exit, time::Instant};

use cubing::{alg::Alg, puzzles::cube3x3x3_kpuzzle};

use clap::{command, Parser};

use crate::{
    patterns::{is_3x3x3_cross_solved, is_slot_solved},
    search::Search,
    triggers::{get_auf_triggers, get_triggers_by_slot},
};

/// Generate a native-style macOS folder icon from a mask file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    scramble: String,
    #[clap(long)]
    debug: bool,
    #[clap(long)]
    no_randomize: bool,
    #[clap(long)]
    no_prefer_immediate_slots: bool,
    #[clap(long, default_value = "9")]
    max_depth: usize,
    /// Defaults to the max depth if not specified.
    #[clap(long)]
    start_depth: Option<usize>,
    #[clap(long, default_value = "3")]
    max_depth_per_slot: usize,
    #[clap(long, default_value = "100")]
    max_num_solutions: usize,
}

pub fn main() {
    let args = Args::parse();
    let kpuzzle = cube3x3x3_kpuzzle();

    let alg = args
        .scramble
        .parse::<Alg>()
        .expect("Invalid input alg syntax.");
    let state = kpuzzle
        .start_state()
        .apply_alg(&alg)
        .expect("Input alg is not valid for puzzle.");

    if !is_3x3x3_cross_solved(&state) {
        eprintln!("The provided alg does not leave cross solved. This is currently unsupported.");
        exit(1)
    }

    let triggers_by_slot = get_triggers_by_slot(&kpuzzle);
    for slot_trigger_info in &triggers_by_slot {
        if is_slot_solved(&state, &slot_trigger_info.f2l_slot) {
            println!(
                "Initially solved slot: {}",
                slot_trigger_info.triggers.iter().as_slice()[0].short_alg
            )
        }
    }

    let max_depth_limit = args.max_depth;
    let start_depth_limit = args.start_depth.unwrap_or(max_depth_limit);

    if start_depth_limit > max_depth_limit {
        eprintln!("Warning: start depth is greater than max depth.")
    }

    let search = Search {
        triggers_by_slot,
        auf_triggers: get_auf_triggers(&kpuzzle),
        debug: args.debug,
        randomize: !args.no_randomize,
        prefer_immediate_slots: !args.no_prefer_immediate_slots,
        start_depth_limit,
        max_depth_limit,
        depth_limit_per_slot: 3,
        max_num_solutions: 10,
    };

    let start = Instant::now();
    search.search(state);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
