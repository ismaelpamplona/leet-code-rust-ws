# p49_group_anagrams
[https://leetcode.com/problems/group-anagrams/](https://leetcode.com/problems/group-anagrams/)

## Initial provided code
```Rust
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - categorize by sorted string in a hashmap

```Rust
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
   let mut map: HashMap<String, Vec<String>> = HashMap::new();

   for i in 0..strs.len() {
      let mut sv: Vec<char> = strs[i].chars().collect();
      sv.sort();
      let ordered_str: String = sv.into_iter().collect();

      let entry = map.entry(ordered_str).or_insert(vec![]);
      entry.push(strs[i].clone());
   }

   return map.values().cloned().collect();     
}
```


- n = length of `strs`
- k = max length of a String in `strs`
- time complexity: $O(n*k*logk)$
- space complexity: $O(n*k)$