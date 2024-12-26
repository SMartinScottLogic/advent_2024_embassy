pub trait Solution {
    type Result;

    fn analyse(&mut self, is_full: bool);

    fn answer_part1(&self, is_full: bool) -> Self::Result;
    fn answer_part2(&self, is_full: bool) -> Self::Result;
}
