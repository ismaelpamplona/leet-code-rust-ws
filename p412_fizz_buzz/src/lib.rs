use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let arr: Vec<i32> = (1..=n).collect();
        let mut result: Vec<String> = vec![];
        for v in arr {
            if v % 3 == 0 && v % 5 == 0 {
                result.push(String::from("FizzBuzz"));
            } else if v % 3 == 0 {
                result.push(String::from("Fizz"));
            } else if v % 5 == 0 {
                result.push(String::from("Buzz"));
            } else {
                result.push(v.to_string());
            }
        }
        result
    }

    pub fn fizz_buzz_conc(n: i32) -> Vec<String> {
        let arr: Vec<i32> = (1..=n).collect();
        let mut result: Vec<String> = vec![];
        for v in arr {
            let mut curr_string = String::from("");
            let div_by_3 = v % 3 == 0;
            let div_by_5 = v % 5 == 0;

            if div_by_3 {
                curr_string = String::from("Fizz");
            }

            if div_by_5 {
                curr_string += "Buzz";
            }

            if curr_string.is_empty() {
                curr_string = v.to_string();
            }
            result.push(curr_string);
        }
        result
    }

    pub fn fizz_buzz_hash(n: i32) -> Vec<String> {
        let arr: Vec<i32> = (1..=n).collect();
        let mut result: Vec<String> = vec![];
        let mut fizz_buzz_map = HashMap::new();
        fizz_buzz_map.insert(3, "Fizz");
        fizz_buzz_map.insert(5, "Buzz");

        let divisors = vec![3, 5];

        for v in arr {
            let mut curr_string = String::from("");
            for d in &divisors {
                if v % d == 0 {
                    curr_string += fizz_buzz_map.get(&d).unwrap_or(&"");
                }
            }

            if curr_string.is_empty() {
                curr_string = v.to_string();
            }

            result.push(curr_string);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let output_1 = Solution::fizz_buzz(3);
        let output_2 = Solution::fizz_buzz_conc(3);
        let output_3 = Solution::fizz_buzz_hash(3);
        assert_eq!(output_1, vec!["1", "2", "Fizz"]);
        assert_eq!(output_2, vec!["1", "2", "Fizz"]);
        assert_eq!(output_3, vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn case_2() {
        let output_1 = Solution::fizz_buzz(5);
        let output_2 = Solution::fizz_buzz_conc(5);
        let output_3 = Solution::fizz_buzz_hash(5);
        assert_eq!(output_1, vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(output_2, vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(output_3, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn case_3() {
        let output_1 = Solution::fizz_buzz(15);
        let output_2 = Solution::fizz_buzz_conc(15);
        let output_3 = Solution::fizz_buzz_hash(15);
        assert_eq!(
            output_1,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
        assert_eq!(
            output_2,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
        assert_eq!(
            output_3,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
