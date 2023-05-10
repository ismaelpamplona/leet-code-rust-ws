use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut rn = ransom_note.clone();
        for m in magazine.chars() {
            for (i, r) in rn.chars().enumerate() {
                if m == r {
                    rn = rn[..i].to_string() + &rn[i + 1..];
                    break;
                }
            }
            if rn.len() == 0 {
                break;
            }
        }
        rn.len() == 0
    }

    pub fn can_construct_hash(ransom_note: String, magazine: String) -> bool {
        let mut magazine_map = HashMap::new();

        for m in magazine.chars() {
            let count = magazine_map.entry(m).or_insert(0);
            *count += 1;
        }

        for r in ransom_note.chars() {
            if let Some(value) = magazine_map.get_mut(&r) {
                if *value > 1 {
                    *value -= 1;
                } else {
                    magazine_map.remove(&r);
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let result_1 = Solution::can_construct(ransom_note.clone(), magazine.clone());
        let result_2 = Solution::can_construct_hash(ransom_note, magazine);
        assert_eq!(result_1, false);
        assert_eq!(result_2, false);
    }

    #[test]
    fn case_02() {
        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let result_1 = Solution::can_construct(ransom_note.clone(), magazine.clone());
        let result_2 = Solution::can_construct_hash(ransom_note, magazine);
        assert_eq!(result_1, false);
        assert_eq!(result_2, false);
    }

    #[test]
    fn case_03() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let result_1 = Solution::can_construct(ransom_note.clone(), magazine.clone());
        let result_2 = Solution::can_construct_hash(ransom_note, magazine);
        assert_eq!(result_1, true);
        assert_eq!(result_2, true);
    }

    #[test]
    fn case_04() {
        let ransom_note = String::from("bg");
        let magazine = String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj");
        let result_1 = Solution::can_construct(ransom_note.clone(), magazine.clone());
        let result_2 = Solution::can_construct_hash(ransom_note, magazine);
        assert_eq!(result_1, true);
        assert_eq!(result_2, true);
    }
}
