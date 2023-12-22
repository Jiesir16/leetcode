use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut dequeue = VecDeque::new();

    while r_idx < nums.len() {
        let r_num = nums[r_idx];
        // 队尾入
        while !dequeue.is_empty() && (dequeue.back().unwrap() - r_num < 0) {
            dequeue.pop_back();
        }
        dequeue.push_back(r_num);


        if r_idx + 1 >= k as usize {
            res.push(*dequeue.front().unwrap());
            // 队头出
            if dequeue.front().unwrap() == &nums[l_idx] {
                dequeue.pop_front();
            }
            l_idx += 1;
        }
        r_idx += 1;
    }


    res
}