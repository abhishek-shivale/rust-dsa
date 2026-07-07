//! https://leetcode.com/problems/kth-largest-element-in-a-stream/
//!
//! Design a class that, given an initial `nums` and `k`, tracks the `k`th
//! largest element as new values are `add`ed one at a time.

pub struct KthLargest {
    k: usize,
    heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let _ = (k, nums);
        todo!()
    }

    pub fn add(&mut self, val: i32) -> i32 {
        let _ = val;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_kth_largest() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
