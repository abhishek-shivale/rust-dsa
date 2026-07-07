//! https://leetcode.com/problems/reconstruct-itinerary/
//!
//! `tickets[i] = [from, to]`. Starting from `"JFK"`, reconstruct the
//! itinerary using every ticket exactly once, choosing the
//! lexicographically smallest valid route.

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(pairs: &[(&str, &str)]) -> Vec<Vec<String>> {
        pairs.iter().map(|(a, b)| vec![a.to_string(), b.to_string()]).collect()
    }

    #[test]
    fn example_1() {
        let tickets = t(&[("MUC", "LHR"), ("JFK", "MUC"), ("SFO", "SJC"), ("LHR", "SFO")]);
        assert_eq!(
            find_itinerary(tickets),
            vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
        );
    }
}
