use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn num_jewels_in_stones_map(jewels: String, stones: String) -> i32 {
        let mut jewels_map: HashMap<char, i32> = HashMap::new();
        let jv: Vec<char> = jewels.chars().collect();
        let sv: Vec<char> = stones.chars().collect();
        let mut ans = 0;

        for i in 0..jv.len() {
            let entry = jewels_map.entry(jv[i]).or_insert(0);
            *entry += 1;
        }

        for i in 0..sv.len() {
            if jewels_map.contains_key(&sv[i]) {
                ans += 1;
            }
        }
        ans
    }

    pub fn num_jewels_in_stones_idiomatic(jewels: String, stones: String) -> i32 {
        stones
            .chars() // Returns an iterator over the chars of a string slice
            .filter(|&stoun| jewels.contains(stoun)) // Creates an iterator which uses a closure to determine if an element should be yielded.
            .count() as i32 // Consumes the iterator, counting the number of iterations and returning it
    }

    pub fn num_jewels_in_stones_set(jewels: String, stones: String) -> i32 {
        let mut jewels_set: HashSet<char> = HashSet::new();
        let jv: Vec<char> = jewels.chars().collect();
        let sv: Vec<char> = stones.chars().collect();
        let mut ans = 0;

        for i in 0..jv.len() {
            jewels_set.insert(jv[i]);
        }

        for i in 0..sv.len() {
            if jewels_set.contains(&sv[i]) {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let jewels = String::from("aA");
        let stones = String::from("aAAbbbb");
        let result1 = Solution::num_jewels_in_stones_map(jewels.clone(), stones.clone());
        let result2 = Solution::num_jewels_in_stones_idiomatic(jewels.clone(), stones.clone());
        let result3 = Solution::num_jewels_in_stones_set(jewels.clone(), stones.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
        assert_eq!(result3, 3);
    }

    #[test]
    fn case_02() {
        let jewels = String::from("z");
        let stones = String::from("ZZ");
        let result1 = Solution::num_jewels_in_stones_map(jewels.clone(), stones.clone());
        let result2 = Solution::num_jewels_in_stones_idiomatic(jewels.clone(), stones.clone());
        let result3 = Solution::num_jewels_in_stones_set(jewels.clone(), stones.clone());
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
    }
}
