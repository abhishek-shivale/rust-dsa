//! https://leetcode.com/problems/first-bad-version/
//!
//! Versions `1..=n` are good, then all versions from some point on are bad.
//! Given `is_bad_version` as an oracle via `first_bad`, and `first` as the
//! actual first bad version for the test harness, find it in O(log n) calls.

pub fn first_bad_version(n: i32, first_bad: i32) -> i32 {
    let is_bad_version = |version: i32| version >= first_bad;
    let _ = is_bad_version;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(first_bad_version(5, 4), 4);
    }

    #[test]
    fn first_is_bad() {
        assert_eq!(first_bad_version(3, 1), 1);
    }
}
