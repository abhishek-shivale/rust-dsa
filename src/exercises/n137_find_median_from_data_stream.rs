//! https://leetcode.com/problems/find-median-from-data-stream/
//!
//! Design a structure that supports adding numbers one at a time and
//! returning the running median.

pub struct MedianFinder {
    // max-heap of the smaller half, min-heap of the larger half
    small: std::collections::BinaryHeap<i32>,
    large: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_num(&mut self, num: i32) {
        let _ = num;
        todo!()
    }

    pub fn find_median(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_running_median() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }
}
