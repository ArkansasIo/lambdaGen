# LambdaGen Algorithm Catalog

This catalog is the target standard-library map, not a claim that every function is implemented today.

| Domain | Initial algorithms | Later expansion |
|---|---|---|
| Arithmetic | gcd, lcm, factorial, powers, modular arithmetic | arbitrary precision, residue systems |
| Number theory | primality, sieve, factorization, Euler phi, divisors | CRT, discrete logarithms, number fields |
| Conjectures | Goldbach, twin primes, Collatz, Legendre, Polignac, Bertrand | configurable bounded-search framework |
| Linear algebra | matrix multiply, Gaussian elimination | LU/QR/SVD, eigenvalues, sparse matrices |
| Calculus | finite differences, Newton, integration | symbolic differentiation, ODE/PDE solvers |
| Optimization | argmin, argmax, gradient descent | simplex, quasi-Newton, constrained optimization |
| Graphs | BFS, DFS, Dijkstra, components | A*, max-flow, matching, centrality |
| Combinatorics | permutations, combinations | partitions, generating functions, enumeration |
| Geometry | distance, area, centroid, convex hull | triangulation, intersections, spatial indexes |
| Statistics | mean, variance, covariance | regression, distributions, hypothesis tests |
| Probability | sampling, expectation | Markov chains, Bayesian inference, Monte Carlo |
| Signal | convolution, FFT | filters, wavelets, spectral estimation |
| Information | entropy | mutual information, coding theory |
| Cryptography | hashes and primitives via vetted libraries | signatures, key exchange, finite-field arithmetic |
| Symbolic | expression trees and simplification | CAS, polynomial algebra, solvers |
| Machine learning | k-means, distance metrics | linear/logistic models, trees, neural-network kernels |
| Strings | reverse, search, edit distance | regex engine, tokenization, parsing |
| Sorting/search | quicksort, mergesort, binary search | heaps, radix sort, selection algorithms |
| Dynamic programming | Fibonacci, LCS, knapsack | sequence alignment, shortest-path DP, memoization |

## Design rule

Each algorithm should have a stable name, domain, complexity notes, accepted input types, deterministic/reproducible behavior, parallelization capability, test vectors, and research provenance.

The registry is metadata-first so the language can discover capabilities before a concrete implementation is loaded. This also gives the eventual compiler and IDE enough information to provide completion, documentation and static diagnostics.
