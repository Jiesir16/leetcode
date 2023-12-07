use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.clone().chars().collect::<Vec<char>>();
    let mut res: i32 = 0;
    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut char_map: HashMap<char, i32> = HashMap::new();

    while r_idx < s.len() {
        let r_char = chars[r_idx];
        *char_map.entry(r_char).or_insert(0) += 1;

        while char_map.get(&r_char).unwrap() > &1 {
            let l_char = chars[l_idx];
            *char_map.entry(l_char).or_insert(1) -= 1;
            l_idx += 1;
        }
        r_idx += 1;
        res = pick_max(res, r_idx as i32 - l_idx as i32);
    }
    println!("char_map:{:?}",char_map);
    res
}

fn pick_max(l: i32, r: i32) -> i32 {
    if l - r > 0 { l } else { r }
}