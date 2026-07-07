//! https://leetcode.com/problems/implement-trie-prefix-tree/
//!
//! Implement a trie with `insert`, `search` (exact word), and
//! `starts_with` (prefix) operations.

pub struct Trie {
    is_word: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, word: String) {
        let _ = word;
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        let _ = word;
        todo!()
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let _ = prefix;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_search_prefix() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
