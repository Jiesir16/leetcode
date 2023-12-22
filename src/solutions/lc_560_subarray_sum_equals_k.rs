use std::collections::HashMap;

#[deprecated]
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut l_idx = 0;
    let mut result = 0;
    while l_idx < nums.len() {
        let mut sum= 0;
        for i in l_idx..nums.len() {
            sum += nums[i];
            if sum == k { result += 1 }
        }
        l_idx += 1;
    }
    result
}

pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
    let mut pre_sum_map = HashMap::new();
    // 给个初始值
    let mut pre_sum = 0;
    pre_sum_map.insert(pre_sum,1);
    let mut result = 0;

    let mut i = 0;
    while i < nums.len() {
        pre_sum += nums[i];
        let pre_key = pre_sum - k;
        if pre_sum_map.contains_key(&pre_key) {
            result += pre_sum_map.get(&pre_key).unwrap();
        }
        *pre_sum_map.entry(pre_sum).or_insert(0) += 1;
        i += 1;
    }

    result
}