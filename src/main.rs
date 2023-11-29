
mod leetcode {
    /// ## 11. 盛最多水的容器
    /// * 输入：[1,8,6,2,5,4,8,3,7]
    /// * 输出：49
    /// ```rust
    /// let args = vec![1,8,6,2,5,4,8,3,7];
    /// let max_area = leetcode::lc_11_container_with_most_water::max_area(args);
    /// println!("---------max_area:{max_area}")
    /// ```
    pub mod lc_11_container_with_most_water;

    ///
    /// 示例 1:
    ///
    /// 输入: nums = [0,1,0,3,12]
    /// 输出: [1,3,12,0,0]
    /// 示例 2:
    ///
    /// 输入: nums = [0]
    /// 输出: [0]
    /// ```rust
    ///    let mut vect = vec![0, 1, 0, 3, 12];
    ///    leetcode::lc_283_move_zeros::move_zeroes(&mut vect);
    ///    println!("{:?}", vect);
    /// ```
    ///
    ///
    pub mod lc_283_move_zeros;

}
fn main() {
    let mut vect = vec![0, 1, 0, 3, 12];
    leetcode::lc_283_move_zeros::move_zeroes(&mut vect);
    println!("{:?}", vect);

    let args = vec![1,8,6,2,5,4,8,3,7];
    let max_area = leetcode::lc_11_container_with_most_water::max_area(args);
    println!("---------max_area:{max_area}")
}



