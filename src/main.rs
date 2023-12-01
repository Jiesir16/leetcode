use leetcode::solutions;

fn main() {
    let mut args = vec![0, 1, 0, 3, 12];
    solutions::lc_283_move_zeros::move_zeroes(&mut args);
    println!("{:?}", args);

    let args = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let max_area = solutions::lc_11_container_with_most_water::max_area(args);
    println!("---------max_area:{max_area}");

    let args = vec![-1, 0, 1, 2, -1, -4];
    let result = solutions::lc_15_3sum::three_sum(args);
    println!("---result:{:?}", result);

    let args = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let result = solutions::lc_42_trapping_rain_water::trap(args);
    println!("---result:{result}");
}