use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..arr.len() {
            let entry = map.entry(arr[i]).or_insert(0);
            *entry += 1;
        }

        for i in 0..arr.len() {
            if let Some(value) = map.get(&arr[i]) {
                if *value == arr[i] {
                    ans = ans.max(*value);
                }
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
        let arr = vec![2, 2, 3, 4];
        let result = Solution::find_lucky(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let result = Solution::find_lucky(arr);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_03() {
        let arr = vec![2, 2, 2, 3, 3];
        let result = Solution::find_lucky(arr);
        assert_eq!(result, -1);
    }
}
