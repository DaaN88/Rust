use std::collections::HashMap;
use std::cmp::max;

fn main() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);

    assert_eq!(length_of_longest_substring("abcabbcbb".to_string()), 3);

    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);

    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hash_map = HashMap::with_capacity(s.len());
    let mut result = 0;
    let mut idx_s = 0;

    for (idx, item) in s.chars().enumerate() {
        match hash_map.get(&item) {
            Some(collision_idx) => {idx_s = max(*collision_idx + 1, idx_s);},
            None => {}
        }

        result = max(result, idx+1 - idx_s);
        hash_map.insert(item, idx);
    }

    result as i32
}
