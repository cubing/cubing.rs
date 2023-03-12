mod triggers;

use std::{process::exit, time::Instant};

use rand::seq::SliceRandom;
use rand::thread_rng;

use cubing::{
    alg::{Alg, AlgBuilder},
    kpuzzle::KState,
    puzzles::cube3x3x3_kpuzzle,
};

use clap::{command, Parser};
use triggers::{get_auf_triggers, get_triggers_by_slot, F2LSlot, SlotTriggerInfo, TriggerInfo};

/// Generate a native-style macOS folder icon from a mask file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    scramble: String,
    #[clap(long)]
    debug: bool,
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

    let search = Search {
        triggers_by_slot,
        auf_triggers: get_auf_triggers(&kpuzzle),
        debug: args.debug,
        start_depth: 12,
        max_depth: 12,
    };

    let start = Instant::now();
    match search.search(&state) {
        Some(result) => {
            let (short, long) = result;
            println!("Solution (HIJK): {}", short);
            println!("Solution (Singmaster): {}", long);
        }
        None => eprintln!("No solution found!"),
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

struct Search {
    triggers_by_slot: Vec<SlotTriggerInfo>,
    auf_triggers: Vec<TriggerInfo>,
    debug: bool,
    start_depth: usize,
    max_depth: usize,
}

struct NextSearch<'a> {
    next_state: KState,
    auf: &'a TriggerInfo,
    trigger: &'a TriggerInfo,
}

impl Search {
    fn search(&self, state: &KState) -> Option<(Alg, Alg)> {
        for remaining_depth in self.start_depth..(self.max_depth + 1) {
            println!("Search depth: {}", remaining_depth);
            if let Some(result) = self.search_recursive_inverse(state, remaining_depth) {
                let (short, long) = result;
                return Some((short.to_alg().invert(), long.to_alg().invert()));
            }
        }
        None
    }

    // TODO: wrap in error?
    fn search_recursive_inverse(
        &self,
        state: &KState,
        remaining_depth: usize,
    ) -> Option<(AlgBuilder, AlgBuilder)> {
        if self.debug {
            // print!("{}", remaining_depth)
        };
        if remaining_depth == 0 {
            if is_f2l_solved(state) {
                return Some((AlgBuilder::default(), AlgBuilder::default()));
            }
            return None;
        }

        let mut num_slots_solved = 0;
        let mut next_searches_preferred = Vec::<NextSearch>::new();
        let mut next_searches_non_preferred = Vec::<NextSearch>::new();
        for slot_trigger_info in &self.triggers_by_slot {
            // TODO: pass this down instead of checking every time.
            if is_slot_solved(state, &slot_trigger_info.f2l_slot) {
                num_slots_solved += 1;
                continue;
            }
            for auf in &self.auf_triggers {
                let new_state = state.apply_transformation(&auf.transformation);
                for trigger in &slot_trigger_info.triggers {
                    let next_state = new_state.apply_transformation(&trigger.transformation);
                    let next_searches = if is_slot_solved(&next_state, &slot_trigger_info.f2l_slot)
                    {
                        // if self.debug {
                        //     println!("Preferred! {} {}", auf.short_alg, trigger.short_alg)
                        // };
                        &mut next_searches_preferred
                    } else {
                        &mut next_searches_non_preferred
                    };
                    next_searches.push(NextSearch {
                        next_state,
                        auf,
                        trigger,
                    })
                }
            }
        }

        next_searches_preferred.shuffle(&mut thread_rng());
        next_searches_non_preferred.shuffle(&mut thread_rng());
        for searches in vec![next_searches_preferred, next_searches_non_preferred] {
            for search in searches {
                if self.debug {
                    for _ in remaining_depth..self.max_depth {
                        print!(" ")
                    }
                    println!("↳ {} {}", search.auf.short_alg, search.trigger.short_alg);
                }
                if let Some(solution) =
                    self.search_recursive_inverse(&search.next_state, remaining_depth - 1)
                {
                    let (mut short, mut long) = solution;
                    short.push(&search.trigger.short_alg.invert());
                    short.push(&search.auf.short_alg.invert());
                    long.push(&search.trigger.long_alg.invert());
                    long.push(&search.auf.long_alg.invert());
                    return Some((short, long));
                }
            }
        }

        // print!("{}{} ", remaining_depth, num_slots_solved);
        if num_slots_solved == 4 {
            return Some((AlgBuilder::default(), AlgBuilder::default()));
        }

        None
    }
}

// TODO: is it more efficient not to borrow `F2LSlot`?
fn is_slot_solved(state: &KState, f2l_slot: &F2LSlot) -> bool {
    // Reid order:
    // UF  UR  UB  UL  . DF  DR  DB  DL  . FR  FL  BR  BL
    // UFR URB UBL ULF . DRF DFL DLB DBR
    // U L F R B D
    match f2l_slot {
        F2LSlot::H => are_slot_pieces_solved(state, 9, 5),
        F2LSlot::I => are_slot_pieces_solved(state, 11, 6),
        F2LSlot::J => are_slot_pieces_solved(state, 10, 7),
        F2LSlot::K => are_slot_pieces_solved(state, 8, 4),
    }
}

fn are_slot_pieces_solved(state: &KState, edge_idx: usize, corner_idx: usize) -> bool {
    is_piece_solved(state, "EDGES", edge_idx) && is_piece_solved(state, "CORNERS", corner_idx)
}

fn is_piece_solved(state: &KState, orbit_name: &str, idx: usize) -> bool {
    let orbit = state
        .state_data
        .get(orbit_name)
        .expect("Invalid 3x3x3 state");
    // TODO: compare against the start state
    orbit.pieces[idx] == idx && orbit.orientation[idx] == 0
}

// TODO: allow comparing to state
fn is_f2l_solved(state: &KState) -> bool {
    let edges = state.state_data.get("EDGES").expect("Invalid 3x3x3 state");
    let corners = state
        .state_data
        .get("CORNERS")
        .expect("Invalid 3x3x3 state");
    let centers = state
        .state_data
        .get("CENTERS")
        .expect("Invalid 3x3x3 state");
    edges.pieces[4..12] == [4, 5, 6, 7, 8, 9, 10, 11]
        && edges.orientation[4..12] == [0, 0, 0, 0, 0, 0, 0, 0]
        && corners.pieces[4..8] == [4, 5, 6, 7]
        && corners.orientation[4..8] == [0, 0, 0, 0]
        && centers.pieces[0..2] == [0, 1] // We can get away with testing just two faces, and don't test orientation
}

fn is_3x3x3_cross_solved(state: &KState) -> bool {
    let edges = state.state_data.get("EDGES").expect("Invalid 3x3x3 state");
    edges.pieces[4..8] == [4, 5, 6, 7] && edges.orientation[4..8] == [0, 0, 0, 0]
}
