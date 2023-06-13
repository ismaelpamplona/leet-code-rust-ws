use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, char> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();

        for i in 0..sv.len() {
            let c1 = sv[i];
            let c2 = tv[i];

            if let Some(value) = map.get(&c1) {
                if *value != c2 {
                    return false;
                }
            } else if set.contains(&c2) {
                return false;
            } else {
                map.insert(c1, c2);
                set.insert(c2);
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
        let s = String::from("egg");
        let t = String::from("add");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("foo");
        let t = String::from("bar");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, false);
    }

    #[test]
    fn case_03() {
        let s = String::from("paper");
        let t = String::from("title");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, true);
    }

    #[test]
    fn case_04() {
        let s = String::from("badc");
        let t = String::from("baba");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, false);
    }

    #[test]
    fn case_05() {
        let s = String::from("aa");
        let t = String::from("ab");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, false);
    }

    #[test]
    fn case_06() {
        let s = String::from("bbbaaaba");
        let t = String::from("aaabbbba");
        let result1 = Solution::is_isomorphic(s.clone(), t.clone());
        assert_eq!(result1, false);
    }
}
