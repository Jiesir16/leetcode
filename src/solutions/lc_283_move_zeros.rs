///
/// ***283.盛最多水的容器***
/// * 示例 1:
///
/// * 输入: nums = [0,1,0,3,12]
/// * 输出: [1,3,12,0,0]
/// ```rust
///    let mut vect = vec![0, 1, 0, 3, 12];
///    solutions::lc_283_move_zeros::move_zeroes(&mut vect);
///    println!("{:?}", vect);
/// ```
///
///
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