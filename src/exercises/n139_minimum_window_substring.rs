//! https://leetcode.com/problems/minimum-window-substring/
//!
//! Return the smallest substring of `s` containing every character of `t`
//! (with multiplicity), or `""` if none exists.

pub fn min_window(s: String, t: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
    }

    #[test]
    fn no_valid_window() {
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
    }
}
