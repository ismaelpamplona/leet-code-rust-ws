#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    fn binary_search(vec: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0 as i32, vec.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if vec[mid as usize] == target {
                println!("do something");
                return mid as i32;
            } else if vec[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        // target is not in arr, but left is at the insertion point
        println!("not found");
        -1
    }

    #[allow(dead_code)]
    fn binary_search_duplicates_left(vec: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as usize;
        let mut right = vec.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if vec[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }

    #[allow(dead_code)]
    fn binary_search_duplicates_right(vec: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as usize;
        let mut right = vec.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if vec[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_binary_search() {
        let vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result1 = Solution::binary_search(vector.clone(), 1);
        let result2 = Solution::binary_search(vector.clone(), 9);
        let result3 = Solution::binary_search(vector.clone(), 11);
        assert_eq!(result1, 1);
        assert_eq!(result2, 9);
        assert_eq!(result3, -1);
    }

    #[test]
    fn case_binary_search_duplicates_left() {
        let vector = vec![0, 1, 1, 3, 4, 5, 6, 7, 9, 9, 10];
        let result1 = Solution::binary_search_duplicates_left(vector.clone(), 1);
        let result2 = Solution::binary_search_duplicates_left(vector.clone(), 9);
        assert_eq!(result1, 1);
        assert_eq!(result2, 8);
    }

    #[test]
    fn case_binary_search_duplicates_right() {
        let vector = vec![0, 1, 1, 3, 4, 5, 6, 7, 9, 9, 10];
        let result1 = Solution::binary_search_duplicates_right(vector.clone(), 1);
        let result2 = Solution::binary_search_duplicates_right(vector.clone(), 9);
        assert_eq!(result1, 3);
        assert_eq!(result2, 10);
    }
}
