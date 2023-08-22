# p1306_jump_game_iii
[https://leetcode.com/problems/jump-game-iii/](https://leetcode.com/problems/jump-game-iii/)

## Initial provided code
```Rust
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        
    }
}
```

## First approach - BFS

- n: number of elements
### time complexity: $O(n)$
- since we will visit every index at most once.

### space complexity: $O(n)$
- since it needs q to store next index. In fact, `q` would keep at most two levels of nodes. 
- since we got two children for each node, the traversal of this solution is a binary tree. 
- the maximum number of nodes within a single level for a binary tree would be $\frac{n}{2}$, so the maximum length of q is $O(\frac{n}{2} + \frac{n}{2}) = O(n)$


