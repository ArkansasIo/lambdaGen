use lambdagen::algorithms::*;

#[test] fn sorts_agree(){let src=[9,2,7,1,5,2];let expected=vec![1,2,2,5,7,9];let mut a=src; bubble_sort(&mut a); assert_eq!(a,expected.as_slice());let mut b=src;heap_sort(&mut b);assert_eq!(b,expected.as_slice());let mut c=src;quick_sort(&mut c);assert_eq!(c,expected.as_slice());}
#[test] fn radix_and_counting_sort(){let mut a=[10u64,1,255,2,0,1000];radix_sort(&mut a);assert_eq!(a,[0,1,2,10,255,1000]);let mut b=[4usize,1,3,1,0,2];counting_sort(&mut b);assert_eq!(b,[0,1,1,2,3,4]);}
#[test] fn search(){let a=[1,3,5,7,9];assert_eq!(binary_search(&a,&7),Some(3));assert_eq!(lower_bound(&a,&6),3);}
#[test] fn graph_algorithms(){let g=vec![vec![1,2],vec![2],vec![3],vec![]];assert_eq!(bfs(&g,0),vec![0,1,2,3]);assert_eq!(topological_sort(&g),Some(vec![0,1,2,3]));}
#[test] fn dynamic_programming(){assert_eq!(fibonacci_dp(10),55);assert_eq!(knapsack_01(&[2,3,4],&[4,5,7],5),9);assert_eq!(lcs_length("lambda","banana"),3);}
