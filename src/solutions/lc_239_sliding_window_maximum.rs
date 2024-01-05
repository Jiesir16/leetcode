use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut dequeue = VecDeque::new();

    while r_idx < nums.len() {
        let r_num = nums[r_idx];
        push_queue_last(&mut dequeue, r_num);

        if r_idx + 1 >= k as usize {
            res.push(*dequeue.front().unwrap());
            pop_queue_front(&mut dequeue, nums[l_idx]);
            l_idx += 1;
        }
        r_idx += 1;
    }
    res
}

/// 队尾入
fn push_queue_last(dequeue: &mut VecDeque<i32>, r_num: i32) {
    while !dequeue.is_empty() && (dequeue.back().unwrap() - r_num < 0) {
        dequeue.pop_back();
    }
    dequeue.push_back(r_num);
}

/// 队头出
fn pop_queue_front(dequeue: &mut VecDeque<i32>, l_num: i32) {
    if dequeue.front().unwrap() == &l_num {
        dequeue.pop_front();
    }
}