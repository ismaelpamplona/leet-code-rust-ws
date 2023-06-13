use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut ans = String::new();

        for c in order.chars() {
            if let Some(&freq) = map.get(&c) {
                ans.push_str(&c.to_string().repeat(freq as usize));
                map.remove(&c);
            }
        }

        for (c, freq) in map {
            ans.push_str(&c.to_string().repeat(freq as usize));
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let order = String::from("cba");
        let s = String::from("abcd");
        let result = Solution::custom_sort_string(order, s);
        assert!(result == String::from("cbad"));
    }

    #[test]
    fn case_02() {
        let order = String::from("cbafg");
        let s = String::from("abcd");
        let result = Solution::custom_sort_string(order, s);
        assert!(result == String::from("cbad"));
    }

    #[test]
    fn case_03() {
        let order = String::from("kqep");
        let s = String::from("pekeq");
        let result = Solution::custom_sort_string(order, s);
        assert!(result == String::from("kqeep"));
    }
}
