struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result = format! {"{}#️⃣{}", result, s};
        }
        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        s.split("#️⃣").skip(1).map(|el| String::from(el)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let codec = Codec::new();
        let dummy_input = vec![String::from("Hello"), String::from("World")];
        let expected = vec![String::from("Hello"), String::from("World")];
        let s = codec.encode(dummy_input);
        let result = codec.decode(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let codec = Codec::new();
        let dummy_input = vec![String::from("")];
        let expected = vec![String::from("")];
        let s = codec.encode(dummy_input);
        let result = codec.decode(s);
        assert_eq!(result, expected);
    }
}
