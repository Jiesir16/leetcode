use std::time::Instant;
use leetcode::solutions;

fn main() {
    let start = Instant::now();
    let result = solutions::lc_239_sliding_window_maximum::max_sliding_window(vec![1,1,1],2);
    println!("用时{:?}",start.elapsed());
    println!("滑动窗口中的最大值数组:{:?}",result);
}