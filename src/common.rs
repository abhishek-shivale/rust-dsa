//! Shared types used across exercises, matching the shapes LeetCode itself
//! hands you in Rust problem signatures. Helpers below only exist to build
//! and inspect these structures from tests.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    /// Build a list from `[1, 2, 3]` -> `1 -> 2 -> 3`.
    pub fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &v in values.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// Flatten a list back into a `Vec` for easy `assert_eq!`.
    pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = head {
            out.push(node.val);
            head = node.next;
        }
        out
    }
}

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    pub fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }

    /// Build a tree from a level-order `Vec<Option<i32>>`, LeetCode style:
    /// `[3, 9, 20, None, None, 15, 7]` -> the usual example tree.
    pub fn from_level_order(values: &[Option<i32>]) -> TreeLink {
        let mut iter = values.iter();
        let root_val = match iter.next() {
            Some(Some(v)) => *v,
            _ => return None,
        };
        let root = TreeNode::new(root_val);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        while let Some(parent) = queue.pop_front() {
            if let Some(Some(v)) = iter.next() {
                let node = TreeNode::new(*v);
                queue.push_back(Rc::clone(&node));
                parent.borrow_mut().left = Some(node);
            }
            if let Some(Some(v)) = iter.next() {
                let node = TreeNode::new(*v);
                queue.push_back(Rc::clone(&node));
                parent.borrow_mut().right = Some(node);
            }
        }

        Some(root)
    }

    /// Flatten a tree back to level order (with `None` gaps) for `assert_eq!`.
    pub fn to_level_order(root: TreeLink) -> Vec<Option<i32>> {
        let mut out = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        if let Some(r) = root {
            queue.push_back(Some(r));
        }
        while let Some(front) = queue.pop_front() {
            match front {
                Some(node) => {
                    let node = node.borrow();
                    out.push(Some(node.val));
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                }
                None => out.push(None),
            }
        }
        // trim trailing `None`s so output matches the compact LeetCode form
        while let Some(None) = out.last() {
            out.pop();
        }
        out
    }
}
