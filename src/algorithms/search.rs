pub fn linear_search<T:PartialEq>(a:&[T],x:&T)->Option<usize>{a.iter().position(|v|v==x)}
pub fn binary_search<T:Ord>(a:&[T],x:&T)->Option<usize>{a.binary_search(x).ok()}
pub fn lower_bound<T:Ord>(a:&[T],x:&T)->usize{a.partition_point(|v|v<x)}
pub fn upper_bound<T:Ord>(a:&[T],x:&T)->usize{a.partition_point(|v|v<=x)}
pub fn interpolation_search(a:&[u64],x:u64)->Option<usize>{if a.is_empty(){return None}let(mut lo,mut hi)=(0,a.len()-1);while lo<=hi&&x>=a[lo]&&x<=a[hi]{if a[lo]==a[hi]{return if a[lo]==x{Some(lo)}else{None}}let pos=lo+(((x-a[lo])as usize)*(hi-lo)/(a[hi]-a[lo])as usize);if a[pos]==x{return Some(pos)}if a[pos]<x{lo=pos+1}else{if pos==0{break}hi=pos-1}}None}
