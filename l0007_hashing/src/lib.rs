use std::collections::HashMap;
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn letters_of_the_english_alphabet(s: String) -> i32 {
        let mut ans = 0;
        let mut hash_map: HashMap<char, i32> = HashMap::new();
        let mut hash_set: HashSet<char> = HashSet::new();

        for (i, c) in ('a'..='z').enumerate() {
            hash_set.insert(c);
            hash_map.insert(c, i as i32 + 1);
            hash_map.insert(c.to_ascii_uppercase(), i as i32 + 1);
        }

        for (i, char) in s.char_indices() {
            if let Some(value) = hash_map.get(&char) {
                ans += (i as i32 + 1) * value;
            }
        }

        ans
    }

    // Example 3: Given an integer array nums,
    // find all the numbers x that satisfy the following:
    // x + 1 is not in nums, and x - 1 is not in nums.
    // https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4511/
    pub fn find_x(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut map: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            map.insert(nums[i] + 1);
            map.insert(nums[i] - 1);
        }

        for i in 0..nums.len() {
            if !map.contains(&nums[i]) {
                result.push(nums[i]);
            }
        }

        result

        // time complexity: O(n)
        // space complexity: O(n)
    }

    // Example 1: You are given a string s and an integer k.
    // Find the length of the longest substring that contains
    // at most k distinct characters. For example, given s = "eceba"
    // and k = 2, return 3. The longest substring with at most 2
    // distinct characters is "ece".
    // https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4512/
    pub fn find_longest_substring(s: String, k: i32) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut left = 0;
        let mut max = 0;
        let sv: Vec<char> = s.chars().collect();

        for right in 0..sv.len() {
            let entry = map.entry(sv[right]).or_insert(0);
            *entry += 1;

            while map.len() as i32 > k {
                let entry = map.entry(sv[left]).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    map.remove(&sv[left]);
                }
                left += 1;
            }

            max = max.max(right - left + 1);
        }
        max as i32

        // time complexity: O(n)
        // space complexity: O(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn case_01() {
    //     let s = String::from("abc");
    //     let result = Solution::letters_of_the_english_alphabet(s);
    //     assert_eq!(result, 14);
    // }

    // #[test]
    // fn case_02() {
    //     let s = String::from("AAA");
    //     let result = Solution::letters_of_the_english_alphabet(s);
    //     assert_eq!(result, 6);
    // }

    // #[test]
    // fn case_03() {
    //     let nums = vec![1, 3, -2, -1, 5, 7];
    //     let result = Solution::find_x(nums);
    //     let expected = vec![1, 3, 5, 7];
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn case_04() {
        let s = String::from("eceba");
        let result = Solution::find_longest_substring(s, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_05() {
        let s = String::from("abcddeedd");
        let result = Solution::find_longest_substring(s, 2);
        assert_eq!(result, 6);
    }
}
