use std::time::Instant;
use leetcode::solutions;

fn main() {
    let start = Instant::now();
    let result = solutions::lc_76_minimum_window_substring::min_window(String::from("abc"),String::from("cba"));
    println!("用时{:?}",start.elapsed());
    println!("最小覆盖字串:{:?}",result);
}