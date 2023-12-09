pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut result = vec![];
    // let s_chars = s.chars().collect::<Vec<char>>();
    // 将p转换成26长度的vec
    let p_key = get_vec(&p);
    let mut l_idx = 0;
    let mut r_idx = p.len() - 1;
    while r_idx < s.len() {
        let sub_str = &s[l_idx..=r_idx];
        if p_key == get_vec(&sub_str.to_string()) {
            result.push(l_idx as i32);
        }
        l_idx += 1;
        r_idx += 1;
    }

    result
}

fn get_vec(p: &String) -> [i32; 26] {
    let mut resut = [0; 26];
    for char_s in p.chars() {
        resut[(char_s as u8 - 'a' as u8) as usize] += 1;
    }
    resut
}