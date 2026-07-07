//! https://leetcode.com/problems/basic-calculator/
//!
//! Evaluate a string expression `s` containing `+`, `-`, parentheses, and
//! non-negative integers (no `*` or `/`).

pub fn calculate(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn nested_parens() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
}
