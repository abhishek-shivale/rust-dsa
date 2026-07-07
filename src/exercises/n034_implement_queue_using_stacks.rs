//! https://leetcode.com/problems/implement-queue-using-stacks/
//!
//! Implement a FIFO queue using only two stacks.

pub struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, x: i32) {
        let _ = x;
        todo!()
    }

    pub fn pop(&mut self) -> i32 {
        todo!()
    }

    pub fn peek(&mut self) -> i32 {
        todo!()
    }

    pub fn empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_order() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
