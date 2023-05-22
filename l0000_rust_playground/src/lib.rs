pub fn playground() {
    let sentence = String::from("Let's go Rust! 🦀");
    let mut str_vec: Vec<char> = vec![];
    for s in sentence.chars() {
        str_vec.push(s);
    }

    let result: String = str_vec.iter().collect();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        playground();
    }
}
