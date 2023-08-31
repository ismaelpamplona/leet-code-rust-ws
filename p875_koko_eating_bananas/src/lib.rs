struct Solution;
impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        fn check(piles: &Vec<i32>, k: f64, h: i32) -> bool {
            let mut hours: f64 = 0.0;
            for bananas in piles {
                hours += (*bananas as f64 / k).ceil();
            }
            hours as i32 <= h
        }
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        while left <= right {
            let mut mid = (left + right) / 2;
            if check(&piles, mid as f64, h) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let result1 = Solution::min_eating_speed(piles.clone(), h);
        assert_eq!(result1, 4);
    }

    #[test]
    fn case_02() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let result1 = Solution::min_eating_speed(piles.clone(), h);
        assert_eq!(result1, 30);
    }

    #[test]
    fn case_03() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        let result1 = Solution::min_eating_speed(piles.clone(), h);
        assert_eq!(result1, 23);
    }
}
