///
/// ***11. 盛最多水的容器***
/// * 输入：[1,8,6,2,5,4,8,3,7]
/// * 输出：49
/// ```rust
/// let args = vec![1,8,6,2,5,4,8,3,7];
/// let max_area = lc::lc_11_container_with_most_water::max_area(args);
/// println!("---------max_area:{max_area}")
/// ```
///

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut x = 1;
    let mut y = height.len();
    let mut max_area = 0;
    while x < y {
        let width = (y - x) as i32;
        let tmp_max_area = min(height[x - 1], height[y - 1]) * width;
        if tmp_max_area > max_area { max_area = tmp_max_area }
        if height[x - 1] < height[y - 1] { x += 1 } else { y -= 1 }
    }
    max_area
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}
