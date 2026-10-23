# LambdaGen Research and Development Roadmap

## 1. Language core
- Lexer, parser, AST and bytecode/VM layers.
- Lua-like functions and tables plus object-oriented classes/instances.
- Sentence/paragraph syntax that compiles to ordinary AST statements.
- Native Rust function registry with type-safe argument/result conversion.

## 2. Mathematics
- Arithmetic, number theory, sequences, statistics, predicates and computational conjecture experiments.
- Explicit distinction between finite computational evidence and mathematical proof.

## 3. Algorithm laboratory
- Sorting/searching, graph theory, dynamic programming, combinatorics, geometry, optimization and text processing.
- Benchmark every algorithm against a reference implementation and representative adversarial inputs.
- Property tests: sortedness, permutation preservation, graph invariants, numerical bounds and deterministic replay.

## 4. Scientific-computing layer
- Matrices/vectors, numerical integration, root finding, interpolation, FFT and differential-equation solvers.
- Probability distributions, random sampling, statistical estimation and Monte Carlo experiments.

## 5. AI and neural-computing layer
- Tensor abstractions, automatic differentiation, activation functions, loss functions, optimizers and attention primitives.
- Neural-network graph execution should be a separate subsystem from the language VM.
- Future synthetic-brain/CL1 research should expose experiments through a reproducible data/model interface rather than hard-code biological claims into the language runtime.

## 6. Systems layer
- Serialization, configuration, logging, deterministic seeds, profiling, plugin interfaces and sandboxed native calls.
- Optional parallel execution and accelerator backends after the single-threaded semantics are stable.

## 7. Research governance
- Every research algorithm gets a source note, assumptions, numerical limits, complexity statement and test vectors.
- Experimental results must record input ranges and implementation version.
- Open mathematical conjectures must never be labeled proven merely because a finite search succeeds.

## 8. Long-term direction
LambdaGen can become a research-oriented language where the same program can express mathematics, algorithms, simulations and experiments, while keeping the runtime deterministic and the native library modular.
