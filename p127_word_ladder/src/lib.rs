use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn is_valid(word_a: &Vec<char>, word_b: &Vec<char>) -> bool {
            let mut changes = 0;
            for i in 0..word_a.len() {
                if word_a[i] != word_b[i] {
                    changes += 1;
                }
                if changes > 1 {
                    return false;
                }
            }
            true
        }
        let begin_word: Vec<char> = begin_word.chars().collect();
        let end_word: Vec<char> = end_word.chars().collect();
        let word_list: Vec<Vec<char>> = word_list.iter().map(|x| x.chars().collect()).collect();
        let mut q = VecDeque::from([(&begin_word, 1)]);
        let mut seen = HashSet::new();

        while let Some((word_a, steps)) = q.pop_front() {
            if word_a == &end_word {
                return steps;
            }
            for word_b in &word_list {
                if is_valid(&word_a, word_b) && !seen.contains(&(word_b)) && word_a != word_b {
                    q.push_back((word_b, steps + 1));
                    seen.insert(word_b);
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec![
            String::from("hot"),
            String::from("dot"),
            String::from("dog"),
            String::from("lot"),
            String::from("log"),
            String::from("cog"),
        ];
        let result1 =
            Solution::ladder_length(begin_word.clone(), end_word.clone(), word_list.clone());
        assert_eq!(result1, 5);
    }

    #[test]
    fn case_02() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec![
            String::from("hot"),
            String::from("dot"),
            String::from("dog"),
            String::from("lot"),
            String::from("log"),
        ];
        let result1 =
            Solution::ladder_length(begin_word.clone(), end_word.clone(), word_list.clone());
        assert_eq!(result1, 0);
    }
}
