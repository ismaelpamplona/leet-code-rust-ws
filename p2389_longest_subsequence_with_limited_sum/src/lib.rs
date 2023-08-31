struct Solution;
impl Solution {
    pub fn answer_queries_sort_count(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut answer = vec![-1; queries.len()];
        for i in 0..queries.len() {
            let mut count = 0;
            let mut query = queries[i];
            for j in 0..nums.len() {
                if query >= nums[j] {
                    query -= nums[j];
                    count += 1;
                } else {
                    break;
                }
            }
            answer[i] = count;
        }
        answer
    }

    pub fn answer_queries_binary_search(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        fn binary_search(vec: &Vec<i32>, target: i32) -> i32 {
            let mut left = 0 as i64;
            let mut right = vec.len() as i64 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if vec[mid as usize] == target {
                    return mid as i32 + 1;
                } else if vec[mid as usize] > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            left as i32
        }
        nums.sort();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        let mut answer = vec![-1; queries.len()];
        for i in 0..queries.len() {
            let index = binary_search(&nums, queries[i]);
            answer[i] = index;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        let result1 = Solution::answer_queries_sort_count(nums.clone(), queries.clone());
        let result2 = Solution::answer_queries_binary_search(nums.clone(), queries.clone());
        let expected = vec![2, 3, 4];
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 3, 4, 5];
        let queries = vec![1];
        let result1 = Solution::answer_queries_sort_count(nums.clone(), queries.clone());
        let result2 = Solution::answer_queries_binary_search(nums.clone(), queries.clone());
        let expected = vec![0];
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }
}
