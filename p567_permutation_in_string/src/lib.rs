use std::collections::HashMap;

struct Solution;
impl Solution {
    fn sort(s: &String) -> String {
        let mut sorted_s: Vec<char> = s.chars().collect();
        sorted_s.sort();
        sorted_s.into_iter().collect()
    }

    pub fn check_inclusion_sorting(s1: String, s2: String) -> bool {
        let sorted_s1: String = Solution::sort(&s1);
        let (mut i, mut count) = (0, 0);
        let size = s1.len();

        while (i + size) <= s2.len() {
            let sorted_slice: String = Solution::sort(&s2[i..(size + i)].to_string());
            if sorted_slice == sorted_s1 {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn check_inclusion_map(s1: String, s2: String) -> bool {
        let mut s1v: Vec<char> = s1.chars().collect();
        let mut map_s1: HashMap<char, i32> = HashMap::new();
        let size = s1.len();
        let mut i = 0;

        for c in s1v {
            *map_s1.entry(c).or_insert(0) += 1;
        }

        while (i + size) <= s2.len() {
            let slice_v: &Vec<char> = &s2[i..(size + i)].chars().collect();
            let mut map_slice: HashMap<char, i32> = HashMap::new();
            for c in slice_v {
                *map_slice.entry(*c).or_insert(0) += 1;
            }
            if map_s1 == map_slice {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn check_inclusion_sliding(s1: String, s2: String) -> bool {
        let mut s1v: Vec<char> = s1.chars().collect();
        let mut s2v: Vec<char> = s2.chars().collect();
        let mut map_s1: HashMap<char, i32> = HashMap::new();
        let mut map_slice: HashMap<char, i32> = HashMap::new();
        let size = s1.len();
        let mut i = size;

        if s1v.len() > s2v.len() {
            return false;
        }

        for i in 0..s1v.len() {
            *map_s1.entry(s1v[i]).or_insert(0) += 1;
            *map_slice.entry(s2v[i]).or_insert(0) += 1;
        }

        if map_s1 == map_slice {
            return true;
        }

        while (i) < s2v.len() {
            if let Some(freq) = map_slice.get_mut(&s2v[i - size]) {
                *freq -= 1;
                if *freq == 0 {
                    map_slice.remove(&s2v[i - size]);
                }
            }
            *map_slice.entry(s2v[i]).or_insert(0) += 1;
            if map_s1 == map_slice {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn check_inclusion_sliding_optimized(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_map = [0; 26];
        let mut s2_map = [0; 26];

        let s1v: Vec<char> = s1.chars().collect();
        let s2v: Vec<char> = s2.chars().collect();

        for i in 0..s1v.len() {
            s1_map[(s1v[i] as u8 - b'a') as usize] += 1;
            s2_map[(s2v[i] as u8 - b'a') as usize] += 1;
        }

        let mut count = 0;
        for i in 0..26 {
            if s1_map[i] == s2_map[i] {
                count += 1;
            }
        }

        for i in 0..s2.len() - s1.len() {
            let l = (s2v[i] as u8 - b'a') as usize;
            let r = (s2v[i + s1.len()] as u8 - b'a') as usize;

            if count == 26 {
                return true;
            }

            s2_map[r] += 1;
            if s2_map[r] == s1_map[r] {
                count += 1;
            } else if s2_map[r] == s1_map[r] + 1 {
                count -= 1;
            }

            s2_map[l] -= 1;
            if s2_map[l] == s1_map[l] {
                count += 1;
            } else if s2_map[l] == s1_map[l] - 1 {
                count -= 1;
            }
        }

        count == 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");
        let result1 = Solution::check_inclusion_sorting(s1.clone(), s2.clone());
        let result2 = Solution::check_inclusion_map(s1.clone(), s2.clone());
        let result3 = Solution::check_inclusion_sliding(s1.clone(), s2.clone());
        let result4 = Solution::check_inclusion_sliding_optimized(s1.clone(), s2.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
        assert_eq!(result4, true);
    }

    #[test]
    fn case_02() {
        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");
        let result1 = Solution::check_inclusion_sorting(s1.clone(), s2.clone());
        let result2 = Solution::check_inclusion_map(s1.clone(), s2.clone());
        let result3 = Solution::check_inclusion_sliding(s1.clone(), s2.clone());
        let result4 = Solution::check_inclusion_sliding_optimized(s1.clone(), s2.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }

    #[test]
    fn case_03() {
        let s1 = String::from("adc");
        let s2 = String::from("dcda");
        let result1 = Solution::check_inclusion_sorting(s1.clone(), s2.clone());
        let result2 = Solution::check_inclusion_map(s1.clone(), s2.clone());
        let result3 = Solution::check_inclusion_sliding(s1.clone(), s2.clone());
        let result4 = Solution::check_inclusion_sliding_optimized(s1.clone(), s2.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
        assert_eq!(result4, true);
    }

    #[test]
    fn case_04() {
        let s1 = String::from("hello");
        let s2 = String::from("ooolleoooleh");
        let result1 = Solution::check_inclusion_sorting(s1.clone(), s2.clone());
        let result2 = Solution::check_inclusion_map(s1.clone(), s2.clone());
        let result3 = Solution::check_inclusion_sliding(s1.clone(), s2.clone());
        let result4 = Solution::check_inclusion_sliding_optimized(s1.clone(), s2.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }

    #[test]
    fn case_05() {
        let s1 = String::from("ab");
        let s2 = String::from("a");
        let result1 = Solution::check_inclusion_sorting(s1.clone(), s2.clone());
        let result2 = Solution::check_inclusion_map(s1.clone(), s2.clone());
        let result3 = Solution::check_inclusion_sliding(s1.clone(), s2.clone());
        let result4 = Solution::check_inclusion_sliding_optimized(s1.clone(), s2.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }
}
