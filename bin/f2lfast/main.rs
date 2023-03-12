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
        start_depth: 8,
        max_total_depth: 8,
        max_depth_per_slot: 3,
        max_num_solutions: 10,
    };

    let start = Instant::now();
    search.search(state);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

struct Search {
    triggers_by_slot: Vec<SlotTriggerInfo>,
    auf_triggers: Vec<TriggerInfo>,
    debug: bool,
    start_depth: usize,
    max_total_depth: usize,
    max_depth_per_slot: usize,
    max_num_solutions: usize,
}

struct SearchStatus {
    num_solutions: usize,
}

struct SearchFrame {
    state: KState,
    total_depth: usize,
    slot_depth: usize,
}

struct SearchFrameRecursionInfo<'a> {
    auf: &'a TriggerInfo,
    trigger: &'a TriggerInfo,
    parent: Option<&'a SearchFrameRecursionInfo<'a>>,
}

impl Search {
    fn search(&self, state: KState) {
        for remaining_depth in self.start_depth..(self.max_total_depth + 1) {
            println!("Search depth: {}", remaining_depth);
            let search_status = &mut SearchStatus { num_solutions: 0 };
            let search_frame = &SearchFrame {
                state: state.clone(),
                total_depth: 0,
                slot_depth: 0,
            };
            self.search_recursive(search_status, search_frame, None)
        }
    }

    // TODO: wrap in error?
    fn search_recursive(
        &self,
        search_status: &mut SearchStatus,
        search_frame: &SearchFrame,
        recursion_info: Option<&SearchFrameRecursionInfo>,
    ) {
        if self.debug {
            // print!("{}", remaining_depth)
        };
        if search_frame.total_depth == self.max_total_depth
            || search_frame.slot_depth == self.max_depth_per_slot
        {
            if is_f2l_solved(&search_frame.state) {
                return;
            }
            return;
        }

        let mut num_slots_solved = 0;
        let mut next_frames_preferred = Vec::<(SearchFrame, SearchFrameRecursionInfo)>::new();
        let mut next_frames_non_preferred = Vec::<(SearchFrame, SearchFrameRecursionInfo)>::new();
        for slot_trigger_info in &self.triggers_by_slot {
            // TODO: pass this down instead of checking every time.
            if is_slot_solved(&search_frame.state, &slot_trigger_info.f2l_slot) {
                num_slots_solved += 1;
                continue;
            }
            for auf in &self.auf_triggers {
                let next_state = search_frame.state.apply_transformation(&auf.transformation);
                for trigger in &slot_trigger_info.triggers {
                    let next_state = next_state.apply_transformation(&trigger.transformation);
                    let (next_searches, remaining_depth_for_slot) =
                        if is_slot_solved(&next_state, &slot_trigger_info.f2l_slot) {
                            (&mut next_frames_preferred, 0)
                        } else {
                            (&mut next_frames_non_preferred, search_frame.slot_depth + 1)
                        };
                    next_searches.push((
                        SearchFrame {
                            state: next_state,
                            total_depth: search_frame.total_depth + 1,
                            slot_depth: remaining_depth_for_slot,
                        },
                        SearchFrameRecursionInfo {
                            auf,
                            trigger,
                            parent: recursion_info,
                        },
                    ))
                }
            }
        }

        // print!("{}{} ", remaining_depth, num_slots_solved);
        if num_slots_solved == 4 {
            let (short_solution, long_solution) = self.build_solutions(recursion_info);
            println!("Solution!");
            println!("Short: {}", short_solution);
            println!("Long: {}", long_solution);
            search_status.num_solutions += 1;
            if search_status.num_solutions == self.max_num_solutions {
                return;
            }
        }

        next_frames_preferred.shuffle(&mut thread_rng());
        next_frames_non_preferred.shuffle(&mut thread_rng());
        for next_frames in vec![next_frames_preferred, next_frames_non_preferred] {
            for next_frame in next_frames {
                let (next_frame, recursion_info) = next_frame;
                if self.debug {
                    for _ in 0..next_frame.total_depth {
                        print!(" ")
                    }
                    println!(
                        "↳ {} {}",
                        recursion_info.auf.short_alg, recursion_info.trigger.short_alg
                    );
                }

                self.search_recursive(search_status, &next_frame, Some(&recursion_info));
            }
        }
    }
    // TODO: output via iterator
    fn build_solutions(&self, recursion_info: Option<&SearchFrameRecursionInfo>) -> (Alg, Alg) {
        let mut short_alg_builder = AlgBuilder::default();
        let mut long_alg_builder = AlgBuilder::default();
        self.build_solutions_recursive(
            &mut short_alg_builder,
            &mut long_alg_builder,
            recursion_info,
        );
        (short_alg_builder.to_alg(), long_alg_builder.to_alg())
    }

    // TODO: output via iterator
    #[allow(clippy::only_used_in_recursion)] // TODO: wait wat
    fn build_solutions_recursive(
        &self,
        short_alg_builder: &mut AlgBuilder,
        long_alg_builder: &mut AlgBuilder,
        recursion_info: Option<&SearchFrameRecursionInfo>,
    ) {
        if let Some(child_info) = recursion_info {
            self.build_solutions_recursive(short_alg_builder, long_alg_builder, child_info.parent);
            short_alg_builder.push(&child_info.auf.short_alg);
            short_alg_builder.push(&child_info.trigger.short_alg);
            long_alg_builder.push(&child_info.auf.long_alg);
            long_alg_builder.push(&child_info.trigger.long_alg);
        }
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
