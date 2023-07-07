struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];
        let parts: Vec<&str> = path.split('/').collect();
        for p in parts {
            if p != "" && p != "." && p != ".." {
                stack.push(p);
            } else if p == ".." {
                stack.pop();
            }
        }
        "/".to_owned() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let path = String::from("/home/");
        let output = String::from("/home");
        let result = Solution::simplify_path(path);
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let path = String::from("/../");
        let output = String::from("/");
        let result = Solution::simplify_path(path);
        assert_eq!(result, output);
    }

    #[test]
    fn case_03() {
        let path = String::from("/home//foo/");
        let output = String::from("/home/foo");
        let result = Solution::simplify_path(path);
        assert_eq!(result, output);
    }

    #[test]
    fn case_04() {
        let path = String::from("/a/b///c/.././d/../f/");
        let output = String::from("/a/b/f");
        let result = Solution::simplify_path(path);
        assert_eq!(result, output);
    }
}
