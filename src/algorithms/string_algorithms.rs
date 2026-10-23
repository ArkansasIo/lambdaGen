pub fn reverse(s:&str)->String{s.chars().rev().collect()}
pub fn is_palindrome(s:&str)->bool{let c=s.chars().collect::<Vec<_>>();c.iter().eq(c.iter().rev())}
pub fn levenshtein(a:&str,b:&str)->usize{super::dynamic::edit_distance(a,b)}
pub fn naive_find(haystack:&str,needle:&str)->Option<usize>{if needle.is_empty(){return Some(0)}haystack.find(needle)}
pub fn frequency(s:&str)->std::collections::BTreeMap<char,usize>{let mut m=std::collections::BTreeMap::new();for c in s.chars(){*m.entry(c).or_default()+=1}m}
