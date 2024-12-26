use anyhow::Result;
use clap::Parser;
use std::{env, io::BufRead, str::FromStr};
use tracing::{Level, debug, error, info, span, trace};
use tracing_subscriber::fmt::format::FmtSpan;
use utils::Solution;
use yansi::Paint;

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

pub fn load<T: Solution + Default>(solver: &mut T, filename: &str) -> std::io::Result<()>
where
    <T as utils::Solution>::ParseError: std::fmt::Debug,
{
    let file = std::fs::File::open(filename)?;

    let reader = std::io::BufReader::new(file);
    for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
        debug!(?id, ?line);
        if let Err(e) = solver.update_from_line(id, &line) {
            error!(?e, "parse failure");
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Solver<'a> {
    Day1(day1::Solution),
    Day2(day2::Solution),
    Invalid(&'a str),
}
impl<'a> Solver<'a> {
    fn for_problem(problem: &'a str) -> Self {
        match problem {
            "day1" => Self::Day1(day1::Solution::default()),
            "day2" => Self::Day2(day2::Solution::default()),
            _ => Self::Invalid(problem),
        }
    }
}

impl Solver<'_> {
    fn is_valid(&self) -> bool {
        match self {
            Self::Invalid(_) => false,
            _ => true,
        }
    }
    fn load(&mut self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::open(filename)?;

        let reader = std::io::BufReader::new(file);
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            debug!(?id, ?line);
            let r = match self {
                Solver::Day1(solution) => {
                    solution.update_from_line(id, &line).map_err(|e| e.into())
                }
                Solver::Day2(solution) => {
                    solution.update_from_line(id, &line).map_err(|e| e.into())
                }
                Solver::Invalid(_) => Err(utils::Error::InvalidProblem),
            };
            if let Err(e) = r {
                error!(?e, "parse failure");
            }
        }
        info!(
            "{}{} {}: {:?}",
            Paint::mask("🎄 "),
            Paint::bold(&Paint::green(filename)),
            Paint::bold(&Paint::yellow("solution")),
            self
        );
        Ok(())
    }

    fn run(&mut self, is_full: bool) -> Result<()> {
        match self {
            Solver::Day1(solution) => solution.analyse(is_full),
            Solver::Day2(solution) => solution.analyse(is_full),
            Solver::Invalid(problem) => {
                return Err(anyhow::Error::msg(format!("Invalid problem: {}", problem)));
            }
        }
        match span!(Level::INFO, "part1").in_scope(|| match self {
            Solver::Day1(solution) => solution.answer_part1(is_full),
            Solver::Day2(solution) => solution.answer_part1(is_full),
            Solver::Invalid(_problem) => Err(utils::Error::InvalidProblem),
        }) {
            Ok(r) => info!(
                "{}part1 answer is {}",
                Paint::mask("🎅 "),
                Paint::bold(&Paint::red(&r))
            ),
            Err(e) => error!("{}part1 failed: {}", Paint::mask("🎅 "), e),
        };

        match span!(Level::INFO, "part2").in_scope(|| match self {
            Solver::Day1(solution) => solution.answer_part2(is_full),
            Solver::Day2(solution) => solution.answer_part2(is_full),
            Solver::Invalid(_problem) => Err(utils::Error::InvalidProblem),
        }) {
            Ok(r) => info!(
                "{}part2 answer is {}",
                Paint::mask("🎅 "),
                Paint::bold(&Paint::red(&r))
            ),
            Err(e) => error!("{}part2 failed: {}", Paint::mask("🎅 "), e),
        }

        Ok(())
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Params {
    /// Day of AoC to run
    #[arg(short, long, value_name = "DAY")]
    day: String,
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
fn main() {
    log_init();

    let params = Params::parse();

    trace!(?params);

    let basename = format!("day{}", params.day);

    if !params.skip_samples {
        span!(Level::INFO, "samples").in_scope(|| {
            for suffix in params.sample {
                let mut solver = Solver::for_problem(&basename);
                if solver.is_valid() {
                    let filename = format!("input/{basename}.{suffix}");
                    if let Err(e) = solver.load(&filename) {
                        error!(
                            "{}Failed loading input '{}': {:?}",
                            Paint::mask("🎄 "),
                            filename,
                            e
                        );
                    } else {
                        if let Err(e) = solver.run(false) {
                            error!(
                                "{}Failed running against input '{}': {:?}",
                                Paint::mask("🎄 "),
                                filename,
                                e
                            );
                        }
                    }
                } else {
                    error!("{}Invalid problem '{}'", Paint::mask("🎄 "), basename);
                    return;
                }
            }
        });
    }

    if !params.skip_full {
        span!(Level::INFO, "full").in_scope(|| {
            for suffix in params.full {
                let mut solver = Solver::for_problem(&basename);
                if solver.is_valid() {
                    let filename = format!("input/{basename}.{suffix}");
                    if let Err(e) = solver.load(&filename) {
                        error!(
                            "{}Failed running against input '{}': {:?}",
                            Paint::mask("🎄 "),
                            filename,
                            e
                        );
                    } else {
                        if let Err(e) = solver.run(true) {
                            error!(
                                "{}Failed running against input '{}': {:?}",
                                Paint::mask("🎄 "),
                                filename,
                                e
                            );
                        }
                    }
                } else {
                    error!("{}Invalid problem '{}'", Paint::mask("🎄 "), basename);
                    return;
                }
            }
        });
    }
}
