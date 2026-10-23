# LambdaGen Algorithm Library

LambdaGen is being developed as a general-purpose algorithm runtime rather than only a number/conjecture program.

## Implemented families

### Sorting
Bubble, insertion, selection, shell, merge, quick, heap, counting, radix, stable and unstable sorting.

### Searching
Linear, binary, lower-bound, upper-bound and interpolation search.

### Graphs
BFS, DFS, Dijkstra shortest paths, topological sorting and connected components.

### Dynamic programming
0/1 knapsack, LCS, edit distance and iterative Fibonacci.

### Numeric and number processing
GCD, LCM, integer square root, exponentiation, sieve of Eratosthenes, Fibonacci and bounded Collatz steps.

### Strings
Reverse, palindrome detection, substring search, character frequency and Levenshtein distance.

### Combinatorics
Factorial, permutations, combinations and binomial coefficients.

### Geometry
Euclidean distance, triangle area and orientation/cross-product tests.

### Data and validation
Frequency maps, sorted frequency tables, sortedness, uniqueness and permutation checks.

### Optimization
Argmin, argmax and clamping utilities.

## Research roadmap

The next layers should add union-find and minimum spanning trees; A*, Bellman-Ford and Floyd-Warshall; max-flow/min-cut; tries and balanced trees; hashing and bloom filters; compression; numerical linear algebra; FFT; probability/statistics; constraint solving; SAT/SMT adapters; scheduling; parser/lexer utilities; automatic differentiation; symbolic algebra; and benchmark/property-testing infrastructure.

Every algorithm should carry tests, complexity notes, deterministic behavior where practical, and a Lambda-native registration path. Rust standard-library algorithms remain useful baselines; for example, current Rust documents `sort` as stable and worst-case O(n log n), while `sort_unstable` is in-place and worst-case O(n log n). citeturn0search0
