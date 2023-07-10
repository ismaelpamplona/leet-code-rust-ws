use std::collections::VecDeque;
struct MovingAverage {
    size: i32,
    q: VecDeque<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            size,
            q: VecDeque::from([0]),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.q.push_back(self.q[self.q.len() - 1] + val);
        let sum = self.q[self.q.len() - 1] - self.q[0];
        let div = (self.q.len() - 1) as f64;
        if self.q.len() > self.size as usize {
            self.q.pop_front();
        }
        sum as f64 / div
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut ma: MovingAverage = MovingAverage::new(3);
        let input = vec![1, 10, 3, 5];
        let mut result: Vec<f64> = vec![];
        for n in input {
            result.push(ma.next(n));
        }
        let output: Vec<f64> = vec![1.0, 5.5, 4.666666666666667, 6.0];
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let mut ma: MovingAverage = MovingAverage::new(5);
        let input = vec![
            12009, 1965, -940, -8516, -16446, 7870, 25545, -21028, 18430, -23464,
        ];
        let mut result: Vec<f64> = vec![];
        for n in input {
            result.push(ma.next(n));
        }
        let output: Vec<f64> = vec![
            12009.0,
            6987.0,
            4344.666666666667,
            1129.5,
            -2385.6,
            -3213.4,
            1502.6,
            -2515.0,
            2874.2,
            1470.6,
        ];
        assert_eq!(result, output);
    }
}
