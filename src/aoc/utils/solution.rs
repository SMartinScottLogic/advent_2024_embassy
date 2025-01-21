pub trait Solution {
    fn new() -> impl Solution;
    fn run_sample(&self);
    fn run_full(&self);
}
