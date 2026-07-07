//! https://leetcode.com/problems/meeting-rooms/ (Premium)
//!
//! Return whether a person could attend every meeting in `intervals`
//! (no two meetings overlap).

pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps() {
        assert!(!can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]));
    }

    #[test]
    fn no_overlap() {
        assert!(can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
    }
}
