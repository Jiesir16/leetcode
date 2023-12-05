use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // 定义判断重复
    let mut same_str_map: HashMap<String, Vec<String>> = HashMap::new();

    for str in strs {
        let mut str_vec: Vec<char> = str.chars().collect();
        str_vec.sort_by(|a, b| a.cmp(b));
        let key = str_vec.into_iter().collect::<String>();
        match same_str_map.get(&key) {
            Some(value) => {
                let mut new_value = value.clone();
                new_value.push(str);
                same_str_map.insert(key, new_value);
            }
            _ => {
                let value = vec![str];
                same_str_map.insert(key, value);
            }
        }
    }

    same_str_map.into_iter().map(|(_, value)| value).collect()
}