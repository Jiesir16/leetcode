pub fn max_area(height: Vec<i32>) -> i32 {
    let mut x = 1;
    let mut y = height.len();
    let mut max_area = 0;
    while x < y {
        let width = (y - x) as i32;
        let tmp_max_area = min(height[x - 1], height[y - 1]) * width;
        if tmp_max_area > max_area { max_area = tmp_max_area }
        if height[x-1] < height[y-1] { x += 1 } else { y -= 1 }
    }
    max_area
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}