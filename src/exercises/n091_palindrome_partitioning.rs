//! https://leetcode.com/problems/palindrome-partitioning/
//!
//! Return every way to partition `s` such that each substring is a
//! palindrome.

pub fn partition(s: String) -> Vec<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort();
        v
    }

    #[test]
    fn example_1() {
        let expected = vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ];
        assert_eq!(normalize(partition("aab".to_string())), normalize(expected));
    }
}
