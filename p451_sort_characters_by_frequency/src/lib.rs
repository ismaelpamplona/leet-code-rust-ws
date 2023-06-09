use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freqs: HashMap<char, i32> = HashMap::new();

        for char in s.chars() {
            *freqs.entry(char).or_insert(0) += 1;
        }

        let mut sorted_freqs: Vec<(char, i32)> = freqs.into_iter().collect();
        sorted_freqs.sort_by_key(|&(_, freq)| -freq);

        let mut result = String::new();
        for (char, freq) in sorted_freqs {
            result.push_str(&char.to_string().repeat(freq as usize));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("tree");
        let result = Solution::frequency_sort(s);
        assert!(result == String::from("eert") || result == String::from("eetr"));
    }

    #[test]
    fn case_02() {
        let s = String::from("cccaaa");
        let result = Solution::frequency_sort(s);
        assert!(result == String::from("aaaccc") || result == String::from("cccaaa"));
    }

    #[test]
    fn case_03() {
        let s = String::from("Aabb");
        let result = Solution::frequency_sort(s);
        assert!(result == String::from("bbaA") || result == String::from("bbAa"));
    }
}
