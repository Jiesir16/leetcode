use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {

    if t.len() > s.len() { return String::from("");}

    let mut l_idx = 0;
    let mut r_idx = 0;
    let s_vec = s.clone().chars().collect::<Vec<char>>();
    let target_map = get_char_map_by_str(&t);

    let mut slid_map = HashMap::new();

    let mut res = [0usize; 2];

    while r_idx < s_vec.len() {
        let r_val = s_vec[r_idx];
        *slid_map.entry(r_val).or_insert(0u32) += 1;
        while check_map(&slid_map, &target_map) {

            // 更新长度最短的窗口边界
            if r_idx - l_idx < res[1] - res[0] || res[1] - res[0] == 0 {
                res[0] = l_idx;
                res[1] = r_idx;
            }
            // 左边界滑动，移除元素
            let count_tmp = slid_map.get(&s_vec[l_idx]).or(Option::from(&0u32)).unwrap();
            if count_tmp <= &1u32 {
                slid_map.remove(&s_vec[l_idx]);
            } else {
                *slid_map.entry(s_vec[l_idx]).or_default() -= 1;
            }
            l_idx += 1;
        }
        r_idx += 1;
    }

    let result_str = &s[res[0]..=res[1]];
    if result_str.len() == 1 && !result_str.eq(&t) {
        return String::from("");
    }
    String::from(result_str)
}

/// 获取字符串词频map
fn get_char_map_by_str(str: &String) -> HashMap<char, u32> {
    let str_vec = str.chars().collect::<Vec<char>>();
    let mut char_count_map = HashMap::new();
    str_vec.iter().for_each(|x| {
        *char_count_map.entry(*x).or_insert(0) += 1;
    });
    char_count_map
}

/// 检查a 词频map是否包含 b 词频map
fn check_map(a: &HashMap<char, u32>, b: &HashMap<char, u32>) -> bool {
    let mut res = true;
    for (b_item, b_count) in b {
        if a.get(b_item).or(Option::from(&0u32)).unwrap() < b_count {
            res = false;
            break;
        }
    }
    res
}