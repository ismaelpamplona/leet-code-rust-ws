use std::collections::{HashMap, VecDeque};

struct Solution;
impl Solution {
    fn backtrack(
        index: usize,
        digits: &Vec<char>,
        cur: &mut Vec<char>,
        result: &mut Vec<String>,
        map: &HashMap<i32, &[char]>,
    ) {
        if cur.len() == digits.len() {
            let seq_string: String = (*cur).iter().collect();
            result.push(seq_string);
            return;
        }

        let d = digits[index].to_digit(10).unwrap() as i32;
        let letters = *map.get(&d).unwrap();
        for c in letters {
            cur.push(*c);
            Self::backtrack(index + 1, digits, cur, result, map);
            cur.pop();
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let letters: Vec<char> = ('a'..='z').collect();
        let mut map = HashMap::new();
        let mut start = 0;
        for n in 2..=9 {
            let end = match n {
                7 | 9 => 4,
                _ => 3,
            };
            let seq = &letters[start..start + end];
            map.insert(n, seq);
            start = start + end;
        }
        let mut result = vec![];
        let mut digits_chars: Vec<char> = digits.chars().collect();
        Self::backtrack(0, &digits_chars, &mut vec![], &mut result, &map);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let digits = "23".to_string();
        let result = Solution::letter_combinations(digits);
        let expected = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let digits = "".to_string();
        let result = Solution::letter_combinations(digits);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let digits = "2".to_string();
        let result = Solution::letter_combinations(digits);
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(result, expected);
    }
}
