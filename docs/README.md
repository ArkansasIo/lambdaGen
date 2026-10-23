# LambdaGen Architecture

LambdaGen is being developed as a layered research language rather than as a single monolithic calculator.

## Layers

1. **Language** — lexer, parser, AST, type/logic checking and execution.
2. **Runtime** — values, objects, functions, modules and standard-library calls.
3. **Algorithms** — reusable algorithms with metadata and tests.
4. **Research** — hierarchical domains, experiments, evidence and reproducibility.
5. **Execution** — local parallel workers and future distributed cluster execution.
6. **Persistence** — MySQL schema, research records and job state.
7. **Data** — tables, CSV, HTML and Excel-compatible export.
8. **API** — stable research/job/table route contracts.
9. **Developer tooling** — CLI, package manager, formatter, language server, documentation and GitHub Pages.

## Recommended next implementation order

### 1. Language core

Replace the current statement-only evaluator with a real lexer, recursive-descent parser, AST and bytecode/VM boundary. Preserve the existing simple syntax as a compatibility layer.

### 2. Numeric type system

Add explicit integer, unsigned integer, rational, floating-point, complex, vector, matrix and tensor values. Avoid using `f64` for exact arithmetic.

### 3. Algorithm dispatch

Connect `AlgorithmRegistry` descriptors to executable implementations through a typed dispatch trait. Each implementation should expose complexity and domain metadata.

### 4. Research runner

Use `ExperimentRunner` as the common execution envelope. Add reproducibility metadata, deterministic partitioning, timing, resource usage and result reduction.

### 5. Persistence

Implement the `Database` abstraction against a real MySQL driver. Keep the existing schema as the persistence contract and add migrations/versioning.

### 6. API and jobs

Add an HTTP adapter around `ResearchApi`, authentication/authorization boundaries, job submission, status streaming and result retrieval.

### 7. Scientific stack

Expand the mathematics library domain-by-domain, with property tests and reference vectors. External scientific libraries may be integrated where they provide mature numerical kernels; LambdaGen should not reimplement cryptographic primitives without a compelling reason.

### 8. IDE/tooling

Add formatter, syntax highlighting, language server, REPL, documentation generator and package manager.

## Reproducibility contract

Every research experiment should record:

- algorithm identifier and version;
- source/module version;
- input domain and parameters;
- random seed when randomness is used;
- worker configuration;
- machine/runtime metadata;
- start/end timestamps;
- tested and passed counts;
- counterexample, when present;
- proof/evidence classification;
- serialized result table.

This turns LambdaGen from a collection of calculations into a research execution system.
