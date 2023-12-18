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
