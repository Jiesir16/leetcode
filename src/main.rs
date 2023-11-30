mod leetcode {

    ///
    /// ***11. 盛最多水的容器***
    /// * 输入：[1,8,6,2,5,4,8,3,7]
    /// * 输出：49
    /// ```rust
    /// let args = vec![1,8,6,2,5,4,8,3,7];
    /// let max_area = leetcode::lc_11_container_with_most_water::max_area(args);
    /// println!("---------max_area:{max_area}")
    /// ```
    ///
    pub mod lc_11_container_with_most_water;

    ///
    /// ***283.盛最多水的容器***
    /// * 示例 1:
    ///
    /// * 输入: nums = [0,1,0,3,12]
    /// * 输出: [1,3,12,0,0]
    /// ```rust
    ///    let mut vect = vec![0, 1, 0, 3, 12];
    ///    leetcode::lc_283_move_zeros::move_zeroes(&mut vect);
    ///    println!("{:?}", vect);
    /// ```
    ///
    ///
    pub mod lc_283_move_zeros;

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
    /// let args = vec![-1,0,1,2,-1,-4];
    /// let result = leetcode::lc_15_3sum::three_sum(args);
    /// println!("---result:{:?}",result)
    /// ```
    pub mod lc_15_3sum;
}

fn main() {
    let mut vect = vec![0, 1, 0, 3, 12];
    leetcode::lc_283_move_zeros::move_zeroes(&mut vect);
    println!("{:?}", vect);

    let args = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let max_area = leetcode::lc_11_container_with_most_water::max_area(args);
    println!("---------max_area:{max_area}");

    let args = vec![-1, 0, 1, 2, -1, -4];
    let result = leetcode::lc_15_3sum::three_sum(args);
    println!("---result:{:?}", result)
}



