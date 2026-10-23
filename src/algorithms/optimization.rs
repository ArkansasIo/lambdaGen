pub fn argmin<T:PartialOrd>(items:&[T])->Option<usize>{if items.is_empty(){return None}let mut m=0;for i in 1..items.len(){if items[i]<items[m]{m=i}}Some(m)}
pub fn argmax<T:PartialOrd>(items:&[T])->Option<usize>{if items.is_empty(){return None}let mut m=0;for i in 1..items.len(){if items[i]>items[m]{m=i}}Some(m)}
pub fn clamp<T:Ord>(x:T,lo:T,hi:T)->T{x.max(lo).min(hi)}
