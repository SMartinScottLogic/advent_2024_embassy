use anyhow::Result;
use clap::Parser;
use std::{env, io::BufRead, str::FromStr};
use day5 as lib;
use tracing::{Level, debug, error, info, span, trace};
use tracing_subscriber::fmt::format::FmtSpan;
use utils::Solution;
use yansi::Paint;

const BASENAME: &'static str = "day5";

pub fn log_init() {
    // install global collector configured based on RUST_LOG env var.
    let level =
        env::var("RUST_LOG").map_or(Level::INFO, |v| Level::from_str(&v).unwrap_or(Level::INFO));
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NONE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(level)
        .init();
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Params {
    /// Skip sample input file(s)
    #[arg(short('s'), long)]
    skip_samples: bool,
    /// Skip full input file(s)
    #[arg(short('f'), long)]
    skip_full: bool,
    /// Suffixes to use for sample input files
    #[arg(short('S'), long, default_value = "sample")]
    sample: Vec<String>,
    /// Suffixes to use for full input files
    #[arg(short('F'), long, default_value = "full")]
    full: Vec<String>,
}

fn load(solver: &mut lib::Solution, filename: &str) -> std::io::Result<()> {
    let file = std::fs::File::open(filename)?;

    let reader = std::io::BufReader::new(file);
    for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
        debug!(?id, ?line);
        let r = solver.update_from_line(id, &line);
        if let Err(e) = r {
            error!(?e, "parse failure");
        }
    }
    info!(
        "{}{} {}: {:?}",
        Paint::mask("🎄 "),
        Paint::bold(&Paint::green(filename)),
        Paint::bold(&Paint::yellow("solution")),
        solver
    );
    Ok(())
}

fn run(solver: &mut lib::Solution, is_full: bool) -> Result<()> {
    solver.analyse(is_full);
    span!(Level::INFO, "part1").in_scope(|| match solver.answer_part1(is_full) {
        Ok(r) => info!(
            "{}part1 answer is {}",
            Paint::mask("🎅 "),
            Paint::bold(&Paint::red(&r))
        ),
        Err(e) => error!("{}part1 failed: {}", Paint::mask("🎅 "), e),
    });

    span!(Level::INFO, "part2").in_scope(|| match solver.answer_part2(is_full) {
        Ok(r) => info!(
            "{}part2 answer is {}",
            Paint::mask("🎅 "),
            Paint::bold(&Paint::red(&r))
        ),
        Err(e) => error!("{}part2 failed: {}", Paint::mask("🎅 "), e),
    });

    Ok(())
}

fn main() {
    log_init();

    let params = Params::parse();

    trace!(?params);

    if !params.skip_samples {
        span!(Level::INFO, "samples").in_scope(|| {
            for suffix in params.sample {
                let mut solver = lib::Solution::default();
                let filename = format!("input/{BASENAME}.{suffix}");
                if let Err(e) = load(&mut solver, &filename) {
                    error!(
                        "{}Failed loading input '{}': {:?}",
                        Paint::mask("🎄 "),
                        filename,
                        e
                    );
                } else {
                    if let Err(e) = run(&mut solver, false) {
                        error!(
                            "{}Failed running against input '{}': {:?}",
                            Paint::mask("🎄 "),
                            filename,
                            e
                        );
                    }
                }
            }
        });
    }

    if !params.skip_full {
        span!(Level::INFO, "full").in_scope(|| {
            for suffix in params.full {
                let mut solver = lib::Solution::default();
                let filename = format!("input/{BASENAME}.{suffix}");
                if let Err(e) = load(&mut solver, &filename) {
                    error!(
                        "{}Failed loading input '{}': {:?}",
                        Paint::mask("🎄 "),
                        filename,
                        e
                    );
                } else {
                    if let Err(e) = run(&mut solver, true) {
                        error!(
                            "{}Failed running against input '{}': {:?}",
                            Paint::mask("🎄 "),
                            filename,
                            e
                        );
                    }
                }
            }
        });
    }
}
