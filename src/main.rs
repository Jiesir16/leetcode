use leetcode::solutions;

fn main() {
    let args = vec![0,3,7,2,5,8,4,6,0,1];
    let result = solutions::lc_128_longest_consecutive_sequence::longest_consecutive(args);
    println!("最长连续序列长度是{result}");
}