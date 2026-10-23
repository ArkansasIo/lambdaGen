use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmDomain {
    Arithmetic,
    NumberTheory,
    LinearAlgebra,
    Calculus,
    DifferentialEquations,
    Optimization,
    Graph,
    Combinatorics,
    Symbolic,
    Geometry,
    Statistics,
    Probability,
    Signal,
    Information,
    Cryptography,
    MachineLearning,
    String,
    Sorting,
    Searching,
    DynamicProgramming,
    Validation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmDescriptor {
    pub name: String,
    pub domain: AlgorithmDomain,
    pub description: String,
    pub deterministic: bool,
    pub parallelizable: bool,
}

impl AlgorithmDescriptor {
    pub fn new(name: &str, domain: AlgorithmDomain, description: &str) -> Self {
        Self {
            name: name.into(),
            domain,
            description: description.into(),
            deterministic: true,
            parallelizable: false,
        }
    }

    pub fn parallelizable(mut self, value: bool) -> Self {
        self.parallelizable = value;
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct AlgorithmRegistry {
    algorithms: BTreeMap<String, AlgorithmDescriptor>,
}

impl AlgorithmRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_standard_catalog() -> Self {
        let mut registry = Self::new();
        let entries = [
            ("gcd", AlgorithmDomain::Arithmetic, "Greatest common divisor"),
            ("lcm", AlgorithmDomain::Arithmetic, "Least common multiple"),
            ("is_prime", AlgorithmDomain::NumberTheory, "Primality test"),
            ("factorization", AlgorithmDomain::NumberTheory, "Integer factorization"),
            ("sieve", AlgorithmDomain::NumberTheory, "Prime sieve"),
            ("goldbach", AlgorithmDomain::NumberTheory, "Bounded Goldbach verification"),
            ("twin_primes", AlgorithmDomain::NumberTheory, "Twin-prime search"),
            ("collatz", AlgorithmDomain::NumberTheory, "Collatz trajectory experiment"),
            ("matrix_multiply", AlgorithmDomain::LinearAlgebra, "Dense matrix multiplication"),
            ("gaussian_elimination", AlgorithmDomain::LinearAlgebra, "Linear-system elimination"),
            ("newton", AlgorithmDomain::Calculus, "Newton iteration"),
            ("integrate", AlgorithmDomain::Calculus, "Numerical integration"),
            ("gradient_descent", AlgorithmDomain::Optimization, "First-order optimization"),
            ("bfs", AlgorithmDomain::Graph, "Breadth-first graph traversal"),
            ("dfs", AlgorithmDomain::Graph, "Depth-first graph traversal"),
            ("dijkstra", AlgorithmDomain::Graph, "Shortest paths with nonnegative weights"),
            ("topological_sort", AlgorithmDomain::Graph, "Directed acyclic graph ordering"),
            ("knapsack", AlgorithmDomain::DynamicProgramming, "0/1 knapsack dynamic programming"),
            ("lcs", AlgorithmDomain::DynamicProgramming, "Longest common subsequence"),
            ("levenshtein", AlgorithmDomain::String, "Edit distance"),
            ("quicksort", AlgorithmDomain::Sorting, "Comparison sorting"),
            ("mergesort", AlgorithmDomain::Sorting, "Stable divide-and-conquer sorting"),
            ("binary_search", AlgorithmDomain::Searching, "Ordered collection search"),
            ("combinations", AlgorithmDomain::Combinatorics, "Combination generation/counting"),
            ("permutations", AlgorithmDomain::Combinatorics, "Permutation generation"),
            ("convex_hull", AlgorithmDomain::Geometry, "Planar convex hull"),
            ("statistics", AlgorithmDomain::Statistics, "Descriptive statistical analysis"),
            ("fft", AlgorithmDomain::Signal, "Fast Fourier transform"),
            ("entropy", AlgorithmDomain::Information, "Shannon entropy"),
            ("sha256", AlgorithmDomain::Cryptography, "SHA-256 hashing"),
            ("kmeans", AlgorithmDomain::MachineLearning, "K-means clustering"),
        ];
        for (name, domain, description) in entries {
            registry.register(AlgorithmDescriptor::new(name, domain, description).parallelizable(true));
        }
        registry
    }

    pub fn register(&mut self, descriptor: AlgorithmDescriptor) {
        self.algorithms.insert(descriptor.name.clone(), descriptor);
    }

    pub fn get(&self, name: &str) -> Option<&AlgorithmDescriptor> {
        self.algorithms.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.algorithms.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.algorithms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.algorithms.is_empty()
    }

    pub fn all(&self) -> impl Iterator<Item = &AlgorithmDescriptor> {
        self.algorithms.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_catalog_is_available() {
        let registry = AlgorithmRegistry::with_standard_catalog();
        assert!(registry.len() >= 20);
        assert!(registry.contains("goldbach"));
        assert!(registry.contains("dijkstra"));
    }
}
