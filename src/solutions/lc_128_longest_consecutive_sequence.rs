use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    //let nums_hash_set = nums.into_iter().collect::<HashSet<i32>>();
    let mut nums_hash_set = HashSet::with_capacity(nums.len());
    for num in nums {
        nums_hash_set.insert(num);
    }

    let mut max_length = 0;
    for num in &nums_hash_set {
        let mut tmp_num = num.clone() + 1;
        if !nums_hash_set.contains(&(num - 1)) {
            let mut current_max = 1;

            while nums_hash_set.contains(&tmp_num) {
                current_max += 1;
                tmp_num += 1;
            }
            max_length = pick_max_for_i32(max_length, current_max);
        }
    }

    max_length
}

fn pick_max_for_i32(l: i32, r: i32) -> i32 {
    if l - r > 0 { l } else { r }
}