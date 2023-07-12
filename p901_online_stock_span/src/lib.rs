use std::collections::VecDeque;

struct StockSpanner {
    stack: VecDeque<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            stack: VecDeque::new(),
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut ans = 1;
        while !self.stack.is_empty() && self.stack[self.stack.len() - 1].0 <= price {
            ans += self.stack.pop_back().unwrap().1;
        }
        self.stack.push_back((price, ans));
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut ss = StockSpanner::new();
        let mut result = vec![];
        let input = vec![100, 80, 60, 70, 60, 75, 85];
        for n in input {
            result.push(ss.next(n));
        }
        assert_eq!(result, vec![1, 1, 1, 2, 1, 4, 6]);
    }
}
