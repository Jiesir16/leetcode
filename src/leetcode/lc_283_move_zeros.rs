
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut a = 0;
    for b in 0..nums.len() {
        if nums[b] != 0 {
            let tmp = nums[b];
            nums[b] = nums[a];
            nums[a] = tmp;
            a += 1;
        }
    }

}