pub trait Solution {
    fn new() -> impl Solution;
    fn run_sample(&mut self);
    fn run_full(&mut self);
}
