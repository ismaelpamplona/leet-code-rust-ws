# p1710_maximum_units_on_a_truck
[https://leetcode.com/problems/maximum-units-on-a-truck/](https://leetcode.com/problems/maximum-units-on-a-truck/)

## Initial provided code
```Rust
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        
    }
}
```

## First approach - Greedy

`n`:the number of elements in vector `box_types`
 
### time complexity: $O(n \cdot \log n)$
- Sorting the array `box_types` of size `n`$O(n \cdot \log n)$ time. 
- Then, we iterate over each element in array `box_types` and in worst case, we might end up iterating over all the elements in the array. This gives us time complexity as $O(n \cdot \log n) + O(n) =  $O(n \cdot \log n)$$.

### space complexity: $O(1)$
- as we use constant extra space.






