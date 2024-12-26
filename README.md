# advent_2024

## Setup new day
To setup a new day, ready for solving, run:
```
cargo xtask setup day${N}
```
Where `N` is the day number, for example:
```
cargo xtask setup day2
```
Will setup `day2` (does NOT perform any git actions)

## Running a day
To run the code for a day `{N}`:
```
cargo run --package day{N} --example runner
```
This will (by default):
* read (and analyse) a sample input file: `input/day{N}.sample`
* read (and analyse) a full input file: `input/day{N}.full`

For additional options, run:
```
cargo run --package day{N} --example runner -- --help
```
