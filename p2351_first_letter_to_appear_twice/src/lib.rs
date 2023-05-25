use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut map: HashMap<char, i32> = HashMap::new();
        let sv: Vec<char> = s.chars().collect();

        for i in 0..sv.len() {
            let mut entry = map.entry(sv[i]).or_insert(0);
            *entry += 1;

            if *entry == 2 {
                return sv[i];
            }
        }
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abccbaacz");
        let result = Solution::repeated_character(s);
        assert_eq!(result, 'c');
    }

    #[test]
    fn case_02() {
        let s = String::from("abcdd");
        let result = Solution::repeated_character(s);
        assert_eq!(result, 'd');
    }
}
