//! https://leetcode.com/problems/course-schedule/
//!
//! `prerequisites[i] = [a, b]` means you must take course `b` before `a`.
//! Return whether all `num_courses` can be finished (no cycle).

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cycle() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn has_cycle() {
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
