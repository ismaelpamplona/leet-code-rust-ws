use std::collections::{HashMap, HashSet};
struct Solution;
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        let sv: Vec<char> = s.chars().collect();

        for i in 0..sv.len() {
            let entry = map.entry(sv[i]).or_insert(0);
            *entry += 1;
        }

        let frequencies: HashSet<&i32> = map.values().collect();
        return frequencies.len() == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abacbc");
        let result = Solution::are_occurrences_equal(s);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("aaabb");
        let result = Solution::are_occurrences_equal(s);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let s = String::from("jfdntzwmzrsurunnoezrybmtm");
        let result = Solution::are_occurrences_equal(s);
        assert_eq!(result, false);
    }
}
