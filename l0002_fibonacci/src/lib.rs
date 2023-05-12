struct Solution;
impl Solution {
    pub fn return_nth_fibo_number(n: i32) -> i32 {
        if n <= 1 {
            // base case
            return n;
        } else {
            let one_back = Self::return_nth_fibo_number(n - 1);
            let two_back = Self::return_nth_fibo_number(n - 2);
            return one_back + two_back;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::return_nth_fibo_number(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_02() {
        let result = Solution::return_nth_fibo_number(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_03() {
        let result = Solution::return_nth_fibo_number(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_04() {
        let result = Solution::return_nth_fibo_number(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_05() {
        let result = Solution::return_nth_fibo_number(4);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_06() {
        let result = Solution::return_nth_fibo_number(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_07() {
        let result = Solution::return_nth_fibo_number(20);
        assert_eq!(result, 6765);
    }
}
