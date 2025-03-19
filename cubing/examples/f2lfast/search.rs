// use std::io::{stdout, Write};

use crate::{
    patterns::{is_3x3x3_solved, is_slot_solved, SlotMask},
    triggers::{SlotTriggerInfo, TriggerInfo},
};

use cubing_core::experimental_twizzle_link::{
    experimental_twizzle_link, ExperimentalTwizzleLinkParameters,
};
use rand::seq::SliceRandom;
use rand::thread_rng;

use cubing::{
    alg::{Alg, AlgBuilder, AlgNode, Pause},
    kpuzzle::KPattern,
};

struct SearchStatus {
    num_solutions: usize,
    depth_limit: usize,
}

struct SearchFrame {
    pattern: KPattern,
    solved_slots: SlotMask, // TODO: see if `&SlotMask` is faster?
    total_depth: usize,
    slot_depth: usize,
}

struct SearchFrameRecursionInfo<'a> {
    // TODO: store algs (or fragments) instead of
    auf: &'a TriggerInfo,
    trigger: &'a TriggerInfo,
    solves_slot: bool,
    parent: Option<&'a SearchFrameRecursionInfo<'a>>,
}

#[derive(Clone)]
pub struct Search {
    pub scramble: Alg,
    pub triggers_by_slot: Vec<SlotTriggerInfo>,
    pub auf_triggers: Vec<TriggerInfo>,
    pub debug: bool,
    pub randomize: bool,
    pub prefer_immediate_slots: bool,
    pub start_depth_limit: usize,
    pub max_depth_limit: usize,
    pub depth_limit_per_slot: usize,
    pub max_num_solutions: usize,
}

impl Search {
    pub fn search(&self, pattern: &KPattern) {
        for depth_limit in self.start_depth_limit..(self.max_depth_limit + 1) {
            println!("Search depth: {}", depth_limit);
            let search_status = &mut SearchStatus {
                depth_limit,
                num_solutions: 0,
            };
            let search_frame = &SearchFrame {
                pattern: pattern.clone(),
                solved_slots: SlotMask::from_pattern(pattern),
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
        if search_frame.solved_slots.is_f2l_solved() {
            // let (short_solution, long_solution) =
            //     self.build_solutions(recursion_info, &Alg::default());
            // println!("F2L Solution!");
            // println!("Short: {}", short_solution);
            // println!("Long: {}", long_solution);
            // println!("{}", twizzle_link(&self.scramble, &long_solution));
            // stdout().flush().unwrap();

            for auf in &self.auf_triggers {
                let with_auf = search_frame
                    .pattern
                    .apply_transformation(&auf.transformation);
                if is_3x3x3_solved(&with_auf) {
                    let (short_solution, long_solution) =
                        self.build_solutions(recursion_info, &auf.short_alg);
                    println!();
                    println!("Full Solution!");
                    println!("Short: {}", short_solution);
                    println!("Long: {}", long_solution);
                    println!(
                        "{}",
                        experimental_twizzle_link(ExperimentalTwizzleLinkParameters {
                            setup: Some(&self.scramble),
                            alg: Some(&long_solution),
                            ..Default::default()
                        })
                    );
                    search_status.num_solutions += 1;
                    if search_status.num_solutions == self.max_num_solutions {
                        return; // TODO: halt the search
                    }
                }
            }
            return; // TODO: Do we want to do this?
        }

        if search_frame.total_depth == search_status.depth_limit
            || search_frame.slot_depth == self.depth_limit_per_slot
        {
            return;
        }

        let mut next_frames_preferred = Vec::<(SearchFrame, SearchFrameRecursionInfo)>::new();
        let mut next_frames_non_preferred = Vec::<(SearchFrame, SearchFrameRecursionInfo)>::new();
        for slot_trigger_info in &self.triggers_by_slot {
            if search_frame
                .solved_slots
                .is_slot_solved(&slot_trigger_info.f2l_slot)
            {
                continue;
            }
            for auf in &self.auf_triggers {
                let next_pattern = search_frame
                    .pattern
                    .apply_transformation(&auf.transformation);
                for trigger in &slot_trigger_info.triggers {
                    let next_pattern = next_pattern.apply_transformation(&trigger.transformation);
                    let (next_searches, remaining_depth_for_slot, solves_slot, solved_slots) =
                        if is_slot_solved(&next_pattern, &slot_trigger_info.f2l_slot) {
                            (
                                if self.prefer_immediate_slots {
                                    &mut next_frames_preferred
                                } else {
                                    &mut next_frames_non_preferred
                                },
                                0,
                                true,
                                search_frame
                                    .solved_slots
                                    .set(&slot_trigger_info.f2l_slot, true),
                            )
                        } else {
                            (
                                &mut next_frames_non_preferred,
                                search_frame.slot_depth + 1,
                                false,
                                search_frame.solved_slots.clone(),
                            )
                        };
                    next_searches.push((
                        SearchFrame {
                            pattern: next_pattern,
                            solved_slots,
                            total_depth: search_frame.total_depth + 1,
                            slot_depth: remaining_depth_for_slot,
                        },
                        SearchFrameRecursionInfo {
                            auf,
                            trigger,
                            solves_slot,
                            parent: recursion_info,
                        },
                    ))
                }
            }
        }

        if self.randomize {
            next_frames_preferred.shuffle(&mut thread_rng());
            next_frames_non_preferred.shuffle(&mut thread_rng());
        }
        for next_frames in [next_frames_preferred, next_frames_non_preferred] {
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
    fn build_solutions(
        &self,
        recursion_info: Option<&SearchFrameRecursionInfo>,
        suffix: &Alg,
    ) -> (Alg, Alg) {
        let mut short_alg_builder = AlgBuilder::default();
        let mut long_alg_builder = AlgBuilder::default();
        self.build_solutions_recursive(
            &mut short_alg_builder,
            &mut long_alg_builder,
            recursion_info,
        );
        short_alg_builder.push(suffix);
        long_alg_builder.push(suffix);
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
            if child_info.solves_slot {
                let pause: AlgNode = Pause {}.into();
                short_alg_builder.push(&pause);
                long_alg_builder.push(&pause);
            }
        }
    }
}
