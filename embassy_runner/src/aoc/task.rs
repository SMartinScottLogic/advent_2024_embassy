use super::day7::{run_sample, run_full};

pub struct AocTask {

}

pub fn build() -> AocTask {
    AocTask {}
}

impl AocTask {
pub async fn run(&mut self) {
    run_sample();
    run_full();
}
}