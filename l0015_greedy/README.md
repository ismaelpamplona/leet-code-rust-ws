# Greedy algorithms

- A greedy algorithm is any algorithm that makes the locally optimal decision at every step.
- A decision is local when it considers only the available options at the current step.  It is based on the information it has at the time, and doesn't consider any consequences that may happen in the future from this decision.

    > Most greedy problems will be asking for the maximum or minimum of something, but not always.

- The hard part of a greedy approach is realizing/proving that a greedy strategy actually works. 
- In many problems, a greedy approach may lead to an answer that is very close to the correct answer. 
- But in real life, greedy algorithms can give good approximations with significantly less computation. 
- A good example is the [travelling salesman problem (TSP)](https://en.wikipedia.org/wiki/Travelling_salesman_problem). A greedy approach to TSP yields an answer that is usually only wrong by about 25%, with a time complexity of $O(n²)$. For an exact solution, we haven't found a classical algorithm faster than $O(2^n)$, and many people doubt such an algorithm exists.
- "Greedy" isn't a data structure and it isn't any single algorithm either, but more of a way to approach a problem.
- The concept of "greedy" is extremely general and the **main thing to practice is recognizing when it applies**. Greedy algorithms are usually very efficient, so if you are given a problem that can be solved greedily, it's important to recognize it.