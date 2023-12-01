
/// ***15.三数之和***
/// * ***示例 1***：
///
/// > 输入：nums = [-1,0,1,2,-1,-4]
/// >
/// > 输出：[[-1,-1,2],[-1,0,1]]
/// > > 不同的三元组是 [-1,0,1] 和 [-1,-1,2]
/// > >
/// > > 注意，输出的顺序和三元组的顺序并不重要
/// ```rust
/// use leetcode::solutions;
/// let args = vec![-1,0,1,2,-1,-4];
/// let result = solutions::lc_15_3sum::three_sum(args);
/// println!("---result:{:?}",result)
/// ```
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    // -4,-1,-1,0,1,2
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        // 如果当前数和前面一个数相同，跳过
        if i > 0 && nums[i] == nums[i - 1] { continue; }

        let target = 0 - nums[i];
        let mut r_idx = nums.len() - 1;
        let mut l_idx = i + 1;
        while l_idx < r_idx {
            if l_idx > i + 1 && nums[l_idx] == nums[l_idx - 1] {
                l_idx += 1;
                continue;
            }
            match nums[l_idx] + nums[r_idx] {
                num_sum if num_sum == target => {
                    result.push(vec![0 - target, nums[l_idx], nums[r_idx]]);
                    l_idx += 1
                }
                num_sum if num_sum > target => r_idx -= 1,
                num_sum if num_sum < target => l_idx += 1,
                _ => { panic!("未知异常") }
            }
        }
    }

    result
}