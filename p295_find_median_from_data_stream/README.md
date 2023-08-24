# p295_find_median_from_data_stream
[https://leetcode.com/problems/find-median-from-data-stream/](https://leetcode.com/problems/find-median-from-data-stream/)

## Initial provided code
```Rust
struct MedianFinder {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        
    }
    
    fn add_num(&self, num: i32) {
        
    }
    
    fn find_median(&self) -> f64 {
        
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
```

## First approach - Two Heaps

### time complexity:
- find_median: $O(1)$
- add_num: $O(log n)$, where `n` is the number of times `add_sum` has been called


### space complexity: $O(n)$


