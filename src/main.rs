mod leetcode {
    pub mod lc_11_container_with_most_water;
    pub mod lc_283_move_zeros;
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



