//! https://leetcode.com/problems/group-anagrams/
//!
//! Group strings that are anagrams of each other. Order of groups and
//! order within a group don't matter, so tests sort before comparing.

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn example_1() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(String::from)
            .collect();
        let expected: Vec<Vec<String>> = vec![
            vec!["bat".into()],
            vec!["nat".into(), "tan".into()],
            vec!["ate".into(), "eat".into(), "tea".into()],
        ];
        assert_eq!(normalize(group_anagrams(input)), normalize(expected));
    }
}
