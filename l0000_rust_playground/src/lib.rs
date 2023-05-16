pub fn go_test() {
    fn return_nth_fibo_number(n: i32) -> i32 {
        if n <= 1 {
            // base case
            return n;
        } else {
            let one_back = return_nth_fibo_number(n - 1);
            let two_back = return_nth_fibo_number(n - 2);
            return one_back + two_back;
        }
    }

    println!("{}", return_nth_fibo_number(0)); // 0
    println!("{}", return_nth_fibo_number(1)); // 1
    println!("{}", return_nth_fibo_number(2)); // 1
    println!("{}", return_nth_fibo_number(3)); // 2
    println!("{}", return_nth_fibo_number(4)); // 3
    println!("{}", return_nth_fibo_number(5)); // 5
    println!("{}", return_nth_fibo_number(6)); // 8
    println!("{}", return_nth_fibo_number(8)); // 21
    println!("{}", return_nth_fibo_number(9)); // 34
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        go_test();
    }
}
