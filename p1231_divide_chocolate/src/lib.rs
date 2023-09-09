#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (
            *sweetness.iter().min().unwrap(),
            sweetness.iter().sum::<i32>() / (k + 1),
        );
        while left < right {
            let min_sweet = (left + right + 1) / 2; // mid
            let mut pieces = 0;
            let mut cur_sweet = 0;
            for s in sweetness.iter() {
                cur_sweet += s;
                if cur_sweet >= min_sweet {
                    cur_sweet = 0;
                    pieces += 1;
                }
            }
            if pieces >= (k + 1) {
                left = min_sweet;
            } else {
                right = min_sweet - 1;
            }
        }
        right
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let sweetness = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result1 = Solution::maximize_sweetness(sweetness, 5);
        assert_eq!(result1, 6);
    }

    #[test]
    fn case_02() {
        let sweetness = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
        let result1 = Solution::maximize_sweetness(sweetness, 8);
        assert_eq!(result1, 1);
    }
}
