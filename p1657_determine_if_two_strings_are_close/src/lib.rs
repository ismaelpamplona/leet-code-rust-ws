use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn close_strings_map(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut w1_map: HashMap<char, usize> = HashMap::new();
        for c in word1.chars() {
            *w1_map.entry(c).or_insert(0) += 1;
        }

        let mut w2_map: HashMap<char, usize> = HashMap::new();
        for c in word2.chars() {
            *w2_map.entry(c).or_insert(0) += 1;
        }

        let mut w1_keys: Vec<char> = w1_map.keys().cloned().collect();
        let mut w1_freqs: Vec<usize> = w1_map.values().cloned().collect();
        let mut w2_keys: Vec<char> = w2_map.keys().cloned().collect();
        let mut w2_freqs: Vec<usize> = w2_map.values().cloned().collect();

        w1_keys.sort();
        w2_keys.sort();

        if w1_keys != w2_keys {
            return false;
        }

        w1_freqs.sort();
        w2_freqs.sort();

        w1_freqs == w2_freqs
    }

    pub fn close_strings_vecmap(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut w1_map: Vec<i32> = vec![0; 26];
        let mut w2_map: Vec<i32> = vec![0; 26];

        for c in word1.chars() {
            let index = (c as u8 - b'a') as usize;
            w1_map[index] += 1;
        }

        for c in word2.chars() {
            let index = (c as u8 - b'a') as usize;
            w2_map[index] += 1;
        }

        for i in 0..26 {
            if (w1_map[i] == 0 && w2_map[i] > 0) || (w2_map[i] == 0 && w1_map[i] > 0) {
                return false;
            }
        }

        w1_map.sort();
        w2_map.sort();

        w1_map == w2_map
    }

    pub fn close_strings_bit(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut w1_map: Vec<i32> = vec![0; 26];
        let mut w2_map: Vec<i32> = vec![0; 26];
        let mut w1_bit: i32 = 0;
        let mut w2_bit: i32 = 0;

        for c in word1.chars() {
            let index = (c as u8 - b'a') as usize;
            w1_map[index] += 1;
            w1_bit |= 1 << index;
        }

        for c in word2.chars() {
            let index = (c as u8 - b'a') as usize;
            w2_map[index] += 1;
            w2_bit |= 1 << index;
        }

        if w1_bit != w2_bit {
            return false;
        }

        w1_map.sort();
        w2_map.sort();

        for i in 0..26 {
            if w1_map[i] != w2_map[i] {
                return false;
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
        let word1 = String::from("abc");
        let word2 = String::from("bca");
        let result1 = Solution::close_strings_map(word1.clone(), word2.clone());
        let result2 = Solution::close_strings_vecmap(word1.clone(), word2.clone());
        let result3 = Solution::close_strings_bit(word1.clone(), word2.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn case_02() {
        let word1 = String::from("a");
        let word2 = String::from("aa");
        let result1 = Solution::close_strings_map(word1.clone(), word2.clone());
        let result2 = Solution::close_strings_vecmap(word1.clone(), word2.clone());
        let result3 = Solution::close_strings_bit(word1.clone(), word2.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }

    #[test]
    fn case_03() {
        let word1 = String::from("cabbba");
        let word2 = String::from("abbccc");
        let result1 = Solution::close_strings_map(word1.clone(), word2.clone());
        let result2 = Solution::close_strings_vecmap(word1.clone(), word2.clone());
        let result3 = Solution::close_strings_bit(word1.clone(), word2.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn case_04() {
        let word1 = String::from("uau");
        let word2 = String::from("ssx");
        let result1 = Solution::close_strings_map(word1.clone(), word2.clone());
        let result2 = Solution::close_strings_vecmap(word1.clone(), word2.clone());
        let result3 = Solution::close_strings_bit(word1.clone(), word2.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }
}
