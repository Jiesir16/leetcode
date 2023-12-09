pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    // 结果vec
    let mut result = vec![];
    // 恶心的题，居然有可能p的长度大于s的长度
    if p.len() > s.len() { return result; }
    // 将s转换成char数组，方便后面取字符
    let s_chars = s.chars().collect::<Vec<char>>();
    // p字符串`特征`
    let p_key = get_vec(&p);
    // 滑动窗口的左边界
    let mut l_idx = 0;
    // 滑动窗口的右边界
    let mut r_idx = p.len() - 1;
    // 起始窗口内字符串的 字符特征
    let mut slid_vec = get_vec(&s[l_idx..=r_idx].to_string());
    while r_idx < s_chars.len() {
        // 如果两个串`特征`一致,就记录下左左边界
        if p_key == slid_vec {
            result.push(l_idx as i32);
        }
        // 滑动窗口左边界右移，字符串变化，更新字符串`特征`
        slid_vec[s_chars[l_idx] as usize - 'a' as usize] -= 1;
        l_idx += 1;

        // 滑动窗口右边界右移，字符串变化，更新字符串`特征`
        r_idx += 1;
        // 避免数组越界
        if r_idx < s_chars.len() {
            slid_vec[s_chars[r_idx] as usize - 'a' as usize] += 1;
        }
    }

    result
}

fn get_vec(p: &String) -> [u8; 26] {
    let mut resut = [0; 26];
    for char_s in p.chars() {
        resut[(char_s as u8 - 'a' as u8) as usize] += 1;
    }
    resut
}