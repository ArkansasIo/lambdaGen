//! General-purpose algorithms exposed to the LambdaGen runtime.
//! The modules are intentionally dependency-light so they can later be
//! registered as native Lambda functions.
pub mod sort;
pub mod search;
pub mod graph;
pub mod dynamic;
pub mod numeric;
pub mod string_algorithms;
pub mod combinatorics;
pub mod geometry;
pub mod optimization;
pub mod validation;
pub mod data;
