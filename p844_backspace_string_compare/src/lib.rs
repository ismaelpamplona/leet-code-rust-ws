struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn build(s1: &String) -> String {
            let mut stack: Vec<char> = vec![];
            for c in s1.chars() {
                if c == '#' {
                    stack.pop();
                } else {
                    stack.push(c)
                }
            }
            stack.iter().collect()
        }
        build(&s) == build(&t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("ab#c");
        let t = String::from("ad#c");
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("ab##");
        let t = String::from("c#d#");
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn case_03() {
        let s = String::from("a#c");
        let t = String::from("b");
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, false);
    }
}
