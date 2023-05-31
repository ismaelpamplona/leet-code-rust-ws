use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut ans = text.len() as i32;
        let mut balloon_map: HashMap<char, i32> = HashMap::new();
        // let balloon_map: HashMap<char, i32> = "balloon".chars().map(|c| (c, 0)).collect();
        let mut text_map: HashMap<char, i32> = HashMap::new();
        let b_vec: Vec<char> = "balloon".chars().collect();
        let s_vec: Vec<char> = text.chars().collect();

        for i in 0..b_vec.len() {
            let entry = balloon_map.entry(b_vec[i]).or_insert(0);
            *entry += 1;
        }

        for i in 0..s_vec.len() {
            if balloon_map.contains_key(&s_vec[i]) {
                let entry = text_map.entry(s_vec[i]).or_insert(0);
                *entry += 1;
            }
        }

        for (k, v) in balloon_map {
            if let Some(value) = text_map.get(&k) {
                ans = ans.min(value / v);
            } else {
                return 0;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let text = String::from("nlaebolko");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_02() {
        let text = String::from("loonbalxballpoon");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_03() {
        let text = String::from("leetcode");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_04() {
        let text = String::from("hpitp");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_05() {
        let text = String::from("ballon");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_06() {
        let text = String::from("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw");
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 10);
    }
}
