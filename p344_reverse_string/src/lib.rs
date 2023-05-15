struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut i, mut j) = (0, s.len() - 1);

        while i < j {
            let tmp = s[i];
            s[i] = s[j];
            s[j] = tmp;
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        let mut output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn case_02() {
        let mut input: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let mut output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }
}
