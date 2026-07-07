//! https://leetcode.com/problems/min-stack/
//!
//! Design a stack supporting `push`, `pop`, `top`, and `get_min` all in
//! O(1).

pub struct MinStack {
    stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, val: i32) {
        let _ = val;
        todo!()
    }

    pub fn pop(&mut self) {
        todo!()
    }

    pub fn top(&self) -> i32 {
        todo!()
    }

    pub fn get_min(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_min_through_pops() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}
