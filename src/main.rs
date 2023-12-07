use leetcode::solutions;

fn main() {
    let args = String::from("pwwwkew");
    let result = solutions::lc_3_longest_substring_without_repeating_characters::length_of_longest_substring(args);
    println!("无重复字符的最长子串长度是{result}");
}