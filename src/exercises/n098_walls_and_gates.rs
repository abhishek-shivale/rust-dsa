//! https://leetcode.com/problems/walls-and-gates/ (Premium)
//!
//! `rooms[r][c]` is `-1` (wall), `0` (gate), or `2147483647` (empty room).
//! Fill each empty room with the distance to its nearest gate, in place.
//! Return the mutated grid for easy testing.

pub const INF: i32 = 2147483647;

pub fn walls_and_gates(rooms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let rooms = vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF, -1],
            vec![INF, -1, INF, -1],
            vec![0, -1, INF, INF],
        ];
        let expected = vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ];
        assert_eq!(walls_and_gates(rooms), expected);
    }
}
