use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let conv_str = |s: &String| s.chars().collect::<Vec<char>>();
        let mut q = VecDeque::from([(conv_str(&start_gene), 0)]);
        let mut seen = HashSet::from([conv_str(&start_gene)]);
        while let Some((node, steps)) = q.pop_front() {
            if node == conv_str(&end_gene) {
                return steps;
            }
            for c in vec!['A', 'C', 'G', 'T'] {
                for i in 0..node.len() {
                    let mut neighbor = node.clone();
                    neighbor[i] = c;

                    if !seen.contains(&neighbor) && bank.contains(&(neighbor.iter().collect())) {
                        q.push_back((neighbor.clone(), steps + 1));
                        seen.insert(neighbor);
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let start_gene = String::from("AACCGGTT");
        let end_gene = String::from("AACCGGTA");
        let bank = vec![String::from("AACCGGTA")];
        let result1 = Solution::min_mutation(start_gene.clone(), end_gene.clone(), bank.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_02() {
        let start_gene = String::from("AACCGGTT");
        let end_gene = String::from("AAACGGTA");
        let bank = vec![
            String::from("AACCGGTA"),
            String::from("AACCGCTA"),
            String::from("AAACGGTA"),
        ];
        let result1 = Solution::min_mutation(start_gene.clone(), end_gene.clone(), bank.clone());
        assert_eq!(result1, 2);
    }
}
