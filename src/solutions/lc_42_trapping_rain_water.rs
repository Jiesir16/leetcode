///
/// * 42.接雨水
///
/// https://leetcode.cn/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-100-liked
/// > 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
/// > * ***示例1***:
/// >
/// > ```输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]```
/// >
/// > 输出：6
///
pub fn trap(height: Vec<i32>) -> i32 {
    let mut pre_max = 0;
    let mut after_max = 0;
    let mut water_nums = 0;

    let mut l_idx = 0;
    let mut r_idx = height.len() - 1;

    while l_idx <= r_idx {
        pre_max = pick_max(pre_max, height[l_idx]);
        after_max = pick_max(after_max, height[r_idx]);
        if pre_max <= after_max {
            water_nums += pre_max - height[l_idx];
            l_idx += 1;
        } else {
            water_nums += after_max - height[r_idx];
            // uszie 可能会减成负数
            r_idx -= 1;
        }
    }
    water_nums
}

fn pick_max(l: i32, r: i32) -> i32 {
    match l - r {
        diff if diff > 0 => l,
        diff if diff <= 0 => r,
        _ => panic!("取最大值出现异常情况!"),
    }
}
