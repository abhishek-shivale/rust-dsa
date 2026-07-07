//! https://leetcode.com/problems/design-add-and-search-words-data-structure/
//!
//! Like a trie, but `search` supports `.` as a wildcard matching any single
//! character.

pub struct WordDictionary {
    is_word: bool,
    children: [Option<Box<WordDictionary>>; 26],
}

impl WordDictionary {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_word(&mut self, word: String) {
        let _ = word;
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        let _ = word;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_search() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert!(!wd.search("pad".to_string()));
        assert!(wd.search("bad".to_string()));
        assert!(wd.search(".ad".to_string()));
        assert!(wd.search("b..".to_string()));
    }
}
