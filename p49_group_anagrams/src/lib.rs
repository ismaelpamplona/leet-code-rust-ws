use std::collections::HashMap;

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

    pub fn group_anagrams_cat_by_count(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut result: Vec<Vec<String>> = vec![];

        for i in 0..strs.len() {
            let mut sv: Vec<char> = strs[i].chars().collect();
            let mut count: Vec<i32> = vec![0; 26];
            for c in sv {
                if let Some(index) = c.to_ascii_lowercase().to_digit(36) {
                    count[index as usize - 10] += 1;
                }
            }
            let key_str: String = count
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join("#");

            let entry = map.entry(key_str).or_insert(vec![]);
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
        let result1 = Solution::group_anagrams(strs.clone());
        let result2 = Solution::group_anagrams_cat_by_count(strs);
        let expected = vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ],
        ];
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn case_02() {
        let strs = vec![String::from("")];
        let result1 = Solution::group_anagrams(strs.clone());
        let result2 = Solution::group_anagrams_cat_by_count(strs);
        let expected = vec![vec![String::from("")]];
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn case_03() {
        let strs = vec![String::from("a")];
        let result1 = Solution::group_anagrams(strs.clone());
        let result2 = Solution::group_anagrams_cat_by_count(strs);
        let expected = vec![vec![String::from("a")]];
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }
}
