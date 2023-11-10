use std::collections::HashMap;

fn fibonacci(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        return n;
    } else if let Some(num) = map.get(&n) {
        return *num;
    } else {
        let one_back = fibonacci(n - 1, map);
        let two_back = fibonacci(n - 2, map);
        return one_back + two_back;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = fibonacci(8, &mut HashMap::new());
        println!("{}", result);
    }
}
