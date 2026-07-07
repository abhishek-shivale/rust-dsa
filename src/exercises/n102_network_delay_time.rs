//! https://leetcode.com/problems/network-delay-time/
//!
//! `times[i] = [u, v, w]`: a signal from `u` to `v` takes `w`. Sent from
//! node `k`, return the time for all `n` nodes to receive it, or -1 if
//! impossible.

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        assert_eq!(network_delay_time(times, 4, 2), 2);
    }

    #[test]
    fn unreachable_node() {
        assert_eq!(network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
    }
}
