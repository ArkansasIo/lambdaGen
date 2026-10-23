use std::collections::{BTreeMap,HashMap};
pub fn frequency_map<T:Eq+std::hash::Hash>(items:&[T])->HashMap<&T,usize>{let mut m=HashMap::new();for x in items{*m.entry(x).or_default()+=1}m}
pub fn sorted_frequency<T:Ord+Clone>(items:&[T])->BTreeMap<T,usize>{let mut m=BTreeMap::new();for x in items{*m.entry(x.clone()).or_default()+=1}m}
