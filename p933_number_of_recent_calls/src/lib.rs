use std::collections::VecDeque;

struct RecentCounter {
    q: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter { q: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.q.len() > 0 && self.q[0] < t - 3000 {
            self.q.pop_front();
        }
        self.q.push_back(t);
        self.q.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn case_01() {
        let mut rc: RecentCounter = RecentCounter::new();
        let pings = vec![1, 100, 3001, 3002];
        let mut result = vec![];
        for p in pings {
            result.push(rc.ping(p));
        }
        let output = vec![1, 2, 3, 3];
        assert_eq!(output, result);
    }

    #[test]
    fn case_02() {
        let mut rc: RecentCounter = RecentCounter::new();
        let pings = vec![1, 100, 3001, 4000];
        let mut result = vec![];
        for p in pings {
            result.push(rc.ping(p));
        }
        let output = vec![1, 2, 3, 2];
        assert_eq!(output, result);
    }
}
