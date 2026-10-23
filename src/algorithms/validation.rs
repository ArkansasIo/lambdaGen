pub fn is_sorted<T:Ord>(a:&[T])->bool{a.windows(2).all(|w|w[0]<=w[1])}
pub fn all_unique<T:Ord+Clone>(a:&[T])->bool{let mut v=a.to_vec();v.sort_unstable();v.windows(2).all(|w|w[0]!=w[1])}
pub fn is_permutation<T:Ord+Clone>(a:&[T],b:&[T])->bool{if a.len()!=b.len(){return false}let mut x=a.to_vec();let mut y=b.to_vec();x.sort_unstable();y.sort_unstable();x==y}
