use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..arr.len() {
            let entry = map.entry(arr[i]).or_insert(0);
            *entry += 1;
        }

        let set: HashSet<i32> = map.values().cloned().collect();

        set.len() == map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let arr = vec![1, 2];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, true);
    }
}
