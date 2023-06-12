use std::collections::HashMap;
use std::iter::repeat;

struct Solution;
impl Solution {
    // pub fn frequency_sort(s: String) -> String {
    //     let mut freqs_map: HashMap<char, i32> = HashMap::new();

    //     for char in s.chars() {
    //         *freqs_map.entry(char).or_insert(0) += 1;
    //     }

    //     let mut sorted_freqs: Vec<(char, i32)> = freqs_map.into_iter().collect();
    //     println!("{:?}", sorted_freqs);
    //     sorted_freqs.sort_by_key(|&(_, freq)| -freq);

    //     let mut result = String::new();
    //     for (c, freq) in sorted_freqs {
    //         result.push_str(&c.to_string().repeat(freq as usize));
    //     }

    //     result
    // }

    pub fn frequency_sort(s: String) -> String {
        let mut freqs_map: HashMap<char, i32> = HashMap::new();

        for char in s.chars() {
            *freqs_map.entry(char).or_insert(0) += 1;
        }

        // Make a vector of the tuples (char, freq), sorted by frequency
        let mut freqs: Vec<(char, i32)> = freqs_map.into_iter().collect();
        freqs.sort_by_key(|&(_, freq)| -freq);

        // Convert the freqs into a String
        freqs
            .iter()
            .flat_map(|&(c, freq)| repeat(c).take(freq as usize))
            .collect()
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
