//! https://leetcode.com/problems/task-scheduler/
//!
//! `tasks` are CPU jobs (letters); identical tasks need `n` units of cooldown
//! between runs. Return the minimum total CPU intervals to finish all
//! tasks (idle slots count).

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    }

    #[test]
    fn no_cooldown() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
    }
}
