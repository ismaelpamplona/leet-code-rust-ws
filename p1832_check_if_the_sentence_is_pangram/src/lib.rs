use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut map: HashSet<char> = HashSet::new();
        let sv: Vec<char> = sentence.chars().collect();

        for i in 0..sv.len() {
            map.insert(sv[i]);
        }

        for letter in 'a'..='z' {
            if !map.contains(&letter) {
                println!("{}", letter);
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
        let sentence = String::from("thequickbrownfoxjumpsoverthelazydog");
        let result = Solution::check_if_pangram(sentence);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let sentence = String::from("leetcode");
        let result = Solution::check_if_pangram(sentence);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let sentence = String::from("onrcsnlxckptsxffbyswujpamfltvmdoxovggepknmtacrjkkorjgvgtgaiaudspnpxkwikevmjeephhiyvnoymjwjfopovscbefecnoytjxfwasabwohqujwowmakpyuuqvgfab");
        let result = Solution::check_if_pangram(sentence);
        assert_eq!(result, false);
    }
}
