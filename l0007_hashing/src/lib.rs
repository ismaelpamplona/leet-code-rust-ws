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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_01() {
        let s = String::from("abc");
        let result = Solution::letters_of_the_english_alphabet(s);
        assert_eq!(result, 14);
    }

    // #[test]
    // fn case_02() {
    //     let s = String::from("AAA");
    //     let result = Solution::letters_of_the_english_alphabet(s);
    //     assert_eq!(result, 6);
    // }
}
