use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut set: HashSet<char> = HashSet::new();
        let sv: Vec<char> = s.chars().collect();
        let mut l = 0;

        for r in 0..sv.len() {
            while set.contains(&sv[r]) {
                set.remove(&sv[l]);
                l += 1;
            }
            set.insert(sv[r]);

            ans = ans.max(r - l + 1);
        }

        ans as i32
    }

    pub fn length_of_longest_substring_map(s: String) -> i32 {
        let mut ans = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        let sv: Vec<char> = s.chars().collect();
        let mut l = 0;
        for r in 0..sv.len() {
            if let Some(value) = map.get(&sv[r]) {
                let v = *value + 1;
                l = v.max(l);
            }

            ans = ans.max(r as i32 - l + 1);
            map.insert(sv[r], r as i32);
        }

        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abcabcbb");
        let result1 = Solution::length_of_longest_substring(s.clone());
        let result2 = Solution::length_of_longest_substring_map(s);
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_02() {
        let s = String::from("bbbbb");
        let result1 = Solution::length_of_longest_substring(s.clone());
        let result2 = Solution::length_of_longest_substring_map(s);
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_03() {
        let s = String::from("pwwkew");
        let result1 = Solution::length_of_longest_substring(s.clone());
        let result2 = Solution::length_of_longest_substring_map(s);
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }
}
