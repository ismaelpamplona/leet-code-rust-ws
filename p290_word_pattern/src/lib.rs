use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map: HashMap<char, &str> = HashMap::new();
        let mut set: HashSet<&str> = HashSet::new();
        let pv: Vec<char> = pattern.chars().collect();
        let sv: Vec<&str> = s.split_whitespace().collect();

        if pv.len() != sv.len() {
            return false;
        }

        for i in 0..pv.len() {
            if let Some(value) = map.get(&pv[i]) {
                if *value != sv[i] {
                    return false;
                }
            } else if set.contains(&sv[i]) {
                return false;
            } else {
                map.insert(pv[i], sv[i]);
                set.insert(sv[i]);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn case_04() {
        let pattern = String::from("abba");
        let s = String::from("dog dog dog dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn case_05() {
        let pattern = String::from("aaa");
        let s = String::from("aa aa aa aa");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }
}
