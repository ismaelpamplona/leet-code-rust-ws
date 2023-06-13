# p567_permutation_in_string
[https://leetcode.com/problems/permutation-in-string/](https://leetcode.com/problems/permutation-in-string/)

## Initial provided code
```Rust
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        
    }
}
```

$l_1$ = length of `s1` <br/>
$l_2$ = length of `s2`

## First approach - sorting strings
- time complexity: $O(l_1log(l_1) + (l_2 - l_1)l_1log(l_1))$
- space complexity: $O(l_1)$

## Second approach - map
- time complexity: $O(l_1 + 26l_1(l_2 - l_1))$ - The hashmap contains atmost 26 keys.
- space complexity: $26 O(1) = O(1)$ - Hashmap contains at most 26 key-value pairs.

## Third approach - map + sliding window
- time complexity: $O(l_1 + 26l_1(l_2 - l_1))$ - The hashmap contains atmost 26 keys.
- space complexity: $26 O(1) = O(1)$ - Hashmap contains at most 26 key-value pairs.

## Fourth approach - map + sliding window (optimized)
- time complexity: $O(l_1 + (l_2 - l_1))$ - The hashmap contains atmost 26 keys.
- space complexity: $26 O(1) = O(1)$ - Hashmap contains at most 26 key-value pairs.
