//! https://leetcode.com/problems/evaluate-reverse-polish-notation/
//!
//! Evaluate an arithmetic expression in Reverse Polish Notation, given as
//! `tokens` (numbers and `+ - * /`).

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(s: &[&str]) -> Vec<String> {
        s.iter().map(|t| t.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(eval_rpn(toks(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            eval_rpn(toks(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])),
            22
        );
    }
}
