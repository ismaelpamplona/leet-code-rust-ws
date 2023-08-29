struct Solution;
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        fn get_number(vec: &Vec<i32>) -> i32 {
            let mut result = 0;
            for digit in vec {
                result = result * 10 + digit;
            }
            result
        }
        fn change_digit(vec: &mut Vec<i32>, i: usize) {
            if vec[i] == 6 {
                vec[i] = 9;
            } else {
                vec[i] = 6;
            }
        }
        let mut num_vec = (num.to_string())
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let mut maximum69 = get_number(&num_vec);
        for i in 0..num_vec.len() {
            change_digit(&mut num_vec, i);
            maximum69 = maximum69.max(get_number(&num_vec));
            change_digit(&mut num_vec, i);
        }
        maximum69
    }

    pub fn maximum69_number_b(num: i32) -> i32 {
        let mut char_vec: Vec<char> = (num.to_string()).chars().collect();
        for i in 0..char_vec.len() {
            if char_vec[i] == '6' {
                char_vec[i] = '9';
                break;
            }
        }
        let mut result = 0;
        for c in char_vec {
            let digit = c.to_digit(10).unwrap() as i32;
            result = result * 10 + digit;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result1 = Solution::maximum69_number(9669);
        let result2 = Solution::maximum69_number_b(9669);
        assert_eq!(result1, 9969);
        assert_eq!(result2, 9969);
    }

    #[test]
    fn case_02() {
        let result1 = Solution::maximum69_number(9996);
        let result2 = Solution::maximum69_number_b(9996);
        assert_eq!(result1, 9999);
        assert_eq!(result2, 9999);
    }

    #[test]
    fn case_03() {
        let result1 = Solution::maximum69_number(9999);
        let result2 = Solution::maximum69_number_b(9999);
        assert_eq!(result1, 9999);
        assert_eq!(result2, 9999);
    }
}
