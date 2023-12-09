use leetcode::solutions;

fn main() {

    let args = String::from("cbaebabacd");
    let args2 = String::from("abc");
    let result = solutions::lc_438_find_all_anagrams_in_a_string::find_anagrams(args, args2);

    println!("字符串中所有字母异位词:{:?}",result);
}