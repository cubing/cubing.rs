# `cubing.rs`

Features from [`cubing.js`](https://github.com/cubing/cubing.js) in Rust. Just tinkering for now.

## Structure

See <https://docs.rs/cubing/latest/cubing/> for Rust docs.

A subset of the [`cubing.js` API](https://js.cubing.net/cubing/api/) is implemented under:

- `cubing::alg`
- `cubing::kpuzzle`
- `cubing::puzzles`

Most applications will use `str.parse::<Alg>(…)` and `KPuzzle` as entry points into the API:

```rust
use cubing::{alg::Alg, puzzles::cube3x3x3_kpuzzle};

pub fn main() {
    let kpuzzle = cube3x3x3_kpuzzle();
    let start_state = kpuzzle.start_state();

    let input_alg = "R U R' F' U2 L' U' L F U2"
        .parse::<Alg>()
        .expect("Invalid alg syntax.");

    let input_state = start_state
        .apply_alg(&input_alg)
        .expect("Input alg is not valid for puzzle.");
    println!(
        "The following alg {} the puzzle to its original state (including center orientation): {}",
        if start_state == input_state { "returns" } else { "does NOT return" },
        input_alg,
    )
}
```

## Development

This repository contains a port of [`f2lfast`](https://github.com/cubing/f2lfast) that is useful for exercising functionality (although it's not yet at full speed):

``` shell
cargo run --example f2lfast -- --scramble "U2 F2 U' L' U L D2 F D2 B' D2 B D2 L2 B L2 F"
```

Run tests using:

```shell
make test
```
