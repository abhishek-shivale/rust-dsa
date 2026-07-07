//! https://leetcode.com/problems/insert-delete-getrandom-o1/
//!
//! Design a set supporting `insert`, `remove`, and `get_random` (uniform
//! among current elements) all in average O(1).

pub struct RandomizedSet {
    values: Vec<i32>,
    index_of: std::collections::HashMap<i32, usize>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let _ = val;
        todo!()
    }

    pub fn remove(&mut self, val: i32) -> bool {
        let _ = val;
        todo!()
    }

    pub fn get_random(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_remove_contains() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
        assert!(set.remove(1));
        assert!(!set.remove(1));
        set.insert(2);
        assert_eq!(set.get_random(), 2);
    }
}
