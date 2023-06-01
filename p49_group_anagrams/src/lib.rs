use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut result: Vec<Vec<String>> = vec![];

        for i in 0..strs.len() {
            let mut sv: Vec<char> = strs[i].chars().collect();
            sv.sort();
            let ordered_str: String = sv.into_iter().collect();

            let entry = map.entry(ordered_str).or_insert(vec![]);
            entry.push(strs[i].clone());
        }

        for (k, v) in map {
            let mut sorted = v.clone();
            sorted.sort();
            result.push(sorted);
        }
        result.sort_by(|a, b| {
            let a_len = a.iter().map(|s| s.len()).sum::<usize>();
            let b_len = b.iter().map(|s| s.len()).sum::<usize>();
            a_len.cmp(&b_len)
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let strs = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result = Solution::group_anagrams(strs);
        let expected = vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let strs = vec![String::from("")];
        let result = Solution::group_anagrams(strs);
        let expected = vec![vec![String::from("")]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let strs = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result = Solution::group_anagrams(strs);
        let expected = vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ],
        ];
        assert_eq!(result, expected);
    }
}
