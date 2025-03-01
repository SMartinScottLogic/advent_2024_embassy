# advent_2024_embassy
Run on Pico
* cargo run --features defmt

Run on Local (linux)
* cargo run --target=x86_64-unknown-linux-gnu --bin linuxmain --features log

Run on Local (windows)
* cargo run --target=x86_64-pc-windows-msvc --bin winmain --features log
