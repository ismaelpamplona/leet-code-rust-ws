use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_map: HashMap<char, i32> = HashMap::new();
        let mut t_map: HashMap<char, i32> = HashMap::new();
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        if s_vec.len() != t_vec.len() {
            return false;
        }
        for i in 0..s_vec.len() {
            let mut entry_s = s_map.entry(s_vec[i]).or_insert(0);
            *entry_s += 1;
            let mut entry_t = t_map.entry(t_vec[i]).or_insert(0);
            *entry_t += 1;
        }
        for (k, s_v) in s_map {
            if let Some(&t_v) = t_map.get(&k) {
                if t_v != s_v {
                    return false;
                }
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let result = Solution::is_anagram(s.clone(), t.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("rat");
        let t = String::from("car");
        let result = Solution::is_anagram(s.clone(), t.clone());
        assert_eq!(result, false);
    }
}
