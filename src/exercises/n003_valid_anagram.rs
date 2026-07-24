//! https://leetcode.com/problems/valid-anagram/
//!
//! Return whether `t` is an anagram of `s` (same letters, same counts).

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    // let mut counts = HashMap::with_capacity(s.len());

    // for ch in s.chars() {
    //     *counts.entry(ch).or_insert(0) += 1;
    // }
    // for ch in t.chars() {
    //     *counts.entry(ch).or_insert(0) -= 1;
    // }

    // counts.values().all(|&count| count == 0)

    let s_len = s.len();
    let t_len = t.len();
    if s_len != t_len {
        return false;
    }
    let mut seen_s: HashMap<char, i32> = HashMap::with_capacity(s.len());
    let mut seen_t: HashMap<char, i32> = HashMap::with_capacity(t.len());

    for i in s.chars() {
        let entry = seen_s.entry(i).or_insert(1);
        *entry += 1;
    }

    for j in t.chars() {
        let entry = seen_t.entry(j).or_insert(1);
        *entry += 1;
    }

    for (ch, num) in seen_s {
        if let Some(n) = seen_t.get(&ch) {
            if n == &num {
                continue;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn not_anagram() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
