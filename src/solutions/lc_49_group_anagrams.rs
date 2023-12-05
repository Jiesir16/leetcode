use std::collections::HashMap;

///
/// * ***示例 1:***
///
/// > * 输入: ```strs = ["eat", "tea", "tan", "ate", "nat", "bat"]```
/// > * 输出: ```[["bat"],["nat","tan"],["ate","eat","tea"]]```
/// ```rust
/// use leetcode::solutions;
/// let args: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"].into_iter().map(|s| s.to_string()).collect();
/// let result = solutions::lc_49_group_anagrams::group_anagrams(args);
/// println!("-----result:{:?}", result)
/// ```
///
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // 单词hashmap
    let mut same_str_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    // 遍历字符串
    for str in strs {
        // 获取单词的char slice数组
        let str_vec: Vec<char> = str.chars().collect();
        // 获取单词特征
        let key = get_key(str_vec);
        // 匹配是否在单词hashmap匹配到单词特征
        match same_str_map.get(&key) {
            Some(value) => {
                // 如果单词特征在hash表中,说明是字母异位词
                let mut value1 = value.clone();
                value1.push(str);
                same_str_map.insert(key, value1);
            }
            _ => {
                // 没有获取到的匹配情况,将新词放到hash表中
                let value = vec![str];
                same_str_map.insert(key, value);
            }
        }
    }
    same_str_map.into_iter().map(|(_, value)| value).collect()
}

///
/// 获取hash表的key，26位字母出现频率的slices,用位图不行，出现重复单词就不行了
///
fn get_key(str_vec: Vec<char>) -> [u8; 26] {
    let mut key = [0u8; 26];
    for c in str_vec {
        let idx: u8 = c as u8 - 'a' as u8;
        key[idx as usize] += 1
    }
    key
}