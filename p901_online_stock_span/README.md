# p901_online_stock_span

[https://leetcode.com/problems/online-stock-span/](https://leetcode.com/problems/online-stock-span/)

## Initial provided code
```Rust
struct StockSpanner {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        
    }
    
    fn next(&self, price: i32) -> i32 {
        
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
```

## First approach - Monotonic Stack
- n: number of calls to `next` function
- time complexity: $O(1)$
- space complexity: $O(n)$

