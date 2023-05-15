struct Solution;
impl Solution {
    pub fn combine_sorted(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
        let (mut i, mut j) = (0, 0);
        let mut result: Vec<i32> = vec![];

        while &i < &vec1.len() && &j < &vec2.len() {
            if vec1[i] < vec2[j] {
                result.push(vec1[i]);
                i += 1;
            } else {
                result.push(vec2[j]);
                j += 1;
            }
        }

        if i < vec1.len() {
            result.extend(vec1[i..].iter());
        }

        if j < vec2.len() {
            result.extend(vec2[j..].iter());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let vec1 = vec![1, 4, 7, 20];
        let vec2 = vec![3, 5, 6];
        let expected = vec![1, 3, 4, 5, 6, 7, 20];
        let result = Solution::combine_sorted(&vec1, &vec2);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let vec1 = vec![0, 2, 4, 6, 8, 9, 10, 11];
        let vec2 = vec![1, 3, 5, 7];
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let result = Solution::combine_sorted(&vec1, &vec2);
        assert_eq!(result, expected);
    }
}
