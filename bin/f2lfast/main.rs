mod triggers;

use std::process::exit;

use cubing::{
    alg::Alg,
    kpuzzle::{KPuzzle, KState},
    puzzles::cube3x3x3_kpuzzle,
};

use clap::{command, Parser};
use triggers::get_triggers;

/// Generate a native-style macOS folder icon from a mask file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    alg: String,
}

pub fn main() {
    let args = Args::parse();
    let kpuzzle = cube3x3x3_kpuzzle();

    let alg = args.alg.parse::<Alg>().expect("Invalid input alg syntax.");
    let state = kpuzzle
        .start_state()
        .apply_alg(&alg)
        .expect("Input alg is not valid for puzzle.");

    if !is_3x3x3_cross_solved(&state) {
        eprintln!("The provided alg does not leave cross solved. This is currently unsupported.");
        exit(1)
    }

    match search(&kpuzzle, &state) {
        Ok(result) => {
            let (short, long) = result;
            println!("Solution (HIJK): {}", short);
            println!("Solution (Singmaster): {}", long);
        }
        Err(e) => eprintln!("Search failed! {}", e),
    }
}

fn search(kpuzzle: &KPuzzle, state: &KState) -> Result<(Alg, Alg), String> {
    let triggers = get_triggers(kpuzzle);

    for trigger in triggers {
        if is_f2l_solved(&state.apply_transformation(&trigger.transformation)) {
            return Ok((trigger.HIJK_alg, trigger.expanded_alg));
        }
    }

    Err("No solution found!".into())
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
