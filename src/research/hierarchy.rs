use super::{NodeKind, ResearchTree};

pub fn default_conjecture_tree() -> ResearchTree {
    let mut t = ResearchTree::root("conjectures", "Mathematical Conjectures", "Executable research hierarchy");
    let branches = [
        ("number_theory", "Number Theory"),
        ("analysis", "Analysis"),
        ("algebra", "Algebra"),
        ("geometry", "Geometry"),
        ("combinatorics", "Combinatorics"),
    ];
    for (id, name) in branches { t.add_child("conjectures", id, name, NodeKind::Branch, "Research branch").unwrap(); }
    t.add_child("number_theory", "prime_theory", "Prime Theory", NodeKind::SubBranch, "Prime distribution and gaps").unwrap();
    t.add_child("prime_theory", "twin_primes", "Twin Primes", NodeKind::Module, "Finite search for prime pairs differing by two").unwrap();
    t.add_child("prime_theory", "prime_gaps", "Prime Gaps", NodeKind::Module, "Finite analysis of consecutive-prime gaps").unwrap();
    t.add_child("number_theory", "additive", "Additive Number Theory", NodeKind::SubBranch, "Additive representations of integers").unwrap();
    t.add_child("additive", "goldbach", "Goldbach", NodeKind::Module, "Finite verification of Goldbach representations").unwrap();
    t.add_child("number_theory", "sequences", "Integer Sequences", NodeKind::SubBranch, "Iterative integer dynamics").unwrap();
    t.add_child("sequences", "collatz", "Collatz", NodeKind::Module, "Bounded Collatz trajectory experiments").unwrap();
    t.add_child("analysis", "zeta", "Zeta Theory", NodeKind::SubBranch, "Numerical zeta-function research").unwrap();
    t.add_child("zeta", "riemann", "Riemann Hypothesis", NodeKind::Module, "Finite numerical probes only; not a proof").unwrap();
    t
}
