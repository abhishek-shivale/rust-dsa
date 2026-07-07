//! https://leetcode.com/problems/maximal-rectangle/
//!
//! `matrix` contains `'0'`/`'1'`. Return the area of the largest rectangle
//! containing only `'1'`s.

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_1() {
        let matrix = m(&["10100", "10111", "11111", "10010"]);
        assert_eq!(maximal_rectangle(matrix), 6);
    }
}
