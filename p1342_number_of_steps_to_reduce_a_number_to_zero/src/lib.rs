struct Solution;
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut steps = 0;
        let mut quotient = num;
        while quotient > 0 {
            steps += 1;
            if quotient % 2 == 0 {
                quotient /= 2;
            } else {
                quotient -= 1;
            }
        }
        steps
    }

    pub fn number_of_steps_bitwise(num: i32) -> i32 {
        let mut steps = 0;
        let mut quotient = num;
        while quotient > 0 {
            steps += 1;
            if quotient % 2 == 0 {
                quotient /= 2;
            } else {
                quotient -= 1;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result_1 = Solution::number_of_steps(14);
        let result_2 = Solution::number_of_steps_bitwise(14);
        assert_eq!(result_1, 6);
        assert_eq!(result_2, 6);
    }

    #[test]
    fn case_2() {
        let result_1 = Solution::number_of_steps(8);
        let result_2 = Solution::number_of_steps_bitwise(8);
        assert_eq!(result_1, 4);
        assert_eq!(result_2, 4);
    }

    #[test]
    fn case_3() {
        let result_1 = Solution::number_of_steps(123);
        let result_2 = Solution::number_of_steps_bitwise(123);
        assert_eq!(result_1, 12);
        assert_eq!(result_2, 12);
    }
}
