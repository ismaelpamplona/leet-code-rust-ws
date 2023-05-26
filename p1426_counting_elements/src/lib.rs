use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn count_elements_hashmap(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for i in 0..arr.len() {
            let mut entry = map.entry(arr[i]).or_insert(0);
            *entry += 1;
        }

        for i in 0..arr.len() {
            let key = arr[i] + 1;
            if map.contains_key(&key) {
                if let Some(value) = map.get_mut(&key) {
                    ans += 1;
                    *value -= 1;
                }
            }
        }
        ans
    }

    pub fn count_elements_hashset(arr: Vec<i32>) -> i32 {
        let mut map: HashSet<i32> = HashSet::new();
        let mut ans = 0;

        for i in 0..arr.len() {
            map.insert(arr[i]);
        }

        for i in 0..arr.len() {
            let key = arr[i] + 1;
            if map.contains(&key) {
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
        let arr = vec![1, 2, 3];
        let result = Solution::count_elements_hashmap(arr.clone());
        let result1 = Solution::count_elements_hashset(arr.clone());
        assert_eq!(result, 2);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let arr = vec![1, 1, 3, 3, 5, 5, 7, 7];
        let result = Solution::count_elements_hashmap(arr.clone());
        let result1 = Solution::count_elements_hashset(arr.clone());
        assert_eq!(result, 0);
        assert_eq!(result1, 0);
    }

    #[test]
    fn case_03() {
        let arr = vec![1, 1, 2];
        let result = Solution::count_elements_hashmap(arr.clone());
        let result1 = Solution::count_elements_hashset(arr.clone());
        assert_eq!(result, 2);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_04() {
        let arr = vec![1, 1, 2, 2];
        let result = Solution::count_elements_hashmap(arr.clone());
        let result1 = Solution::count_elements_hashset(arr.clone());
        assert_eq!(result, 2);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_05() {
        let arr = vec![1, 3, 2, 3, 5, 0];
        let result = Solution::count_elements_hashmap(arr.clone());
        let result1 = Solution::count_elements_hashset(arr.clone());
        assert_eq!(result, 3);
        assert_eq!(result1, 3);
    }
}
