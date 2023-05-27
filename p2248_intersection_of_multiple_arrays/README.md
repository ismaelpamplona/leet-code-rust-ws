# p2248_intersection_of_multiple_arrays
[https://leetcode.com/problems/intersection-of-multiple-arrays/](https://leetcode.com/problems/intersection-of-multiple-arrays/)

## Initial provided code
```Rust
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector of vectors as input.

## First approach - hashing

```Rust
let mut map: HashMap<i32, i32> = HashMap::new();

for m in 0..nums.len() {
   for n in 0..nums[i].len() {
       let entry = map.entry(nums[i][j]).or_insert(0);
       *entry += 1;
   }
}

```
- time complexity: $O(n)$
- space complexity: $O(n)$

```Rust
for k in map.keys() {
    if let Some(v) = map.get(k) {
        if *v == nums.len() as i32 {
            result.push(*k);
        }
    }
}
```
Then, there can be at most m elements inside `result` 

- time complexity: $O(m)$
- space complexity: $O(1)$

```Rust
result.sort();
```
- time complexity: $O(nlogn)$
- space complexity: $O(1)$

## Total time and space complexity

- time complexity: $O(n⋅m+m⋅logm)=O(m⋅(n+logm))$
- space complexity: $O(n⋅m)$

