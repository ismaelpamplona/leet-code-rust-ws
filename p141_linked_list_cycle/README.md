# p141_linked_list_cycle
[https://leetcode.com/problems/linked-list-cycle](https://leetcode.com/problems/linked-list-cycle)

## First approach - HashSet

- n = number of nodes
- time complexity: O(n)
- space complexity: O(n)

## Second approach - Floyd's Tortoise and Hare Algorithm (two pointers)
- n = number of nodes
- m = number of nodes in cycle
- time complexity: $O(n)$
- space complexity: $O(n + m) = O(2n) = O(n)$