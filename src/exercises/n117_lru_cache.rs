//! https://leetcode.com/problems/lru-cache/
//!
//! Design a cache of fixed `capacity` with `get` and `put`, evicting the
//! least recently used entry when full.

pub struct LRUCache {
    capacity: usize,
    // most-recent at the back
    order: Vec<i32>,
    map: std::collections::HashMap<i32, i32>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let _ = capacity;
        todo!()
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let _ = key;
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let _ = (key, value);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evicts_least_recently_used() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
