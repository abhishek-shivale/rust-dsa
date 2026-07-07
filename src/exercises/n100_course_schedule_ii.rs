//! https://leetcode.com/problems/course-schedule-ii/
//!
//! Same setup as Course Schedule; return a valid course order, or an empty
//! vec if impossible.

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_order(order: &[i32], num_courses: i32, prereqs: &[Vec<i32>]) -> bool {
        if order.len() as i32 != num_courses {
            return false;
        }
        let pos: std::collections::HashMap<i32, usize> =
            order.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        prereqs.iter().all(|p| pos[&p[0]] > pos[&p[1]])
    }

    #[test]
    fn example_1() {
        let prereqs = vec![vec![1, 0]];
        let order = find_order(2, prereqs.clone());
        assert!(is_valid_order(&order, 2, &prereqs));
    }

    #[test]
    fn impossible() {
        assert_eq!(find_order(2, vec![vec![1, 0], vec![0, 1]]), Vec::<i32>::new());
    }
}
