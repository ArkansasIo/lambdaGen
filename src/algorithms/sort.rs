pub fn bubble_sort<T: Ord>(a: &mut [T]) { for end in (1..a.len()).rev() { let mut swapped=false; for i in 0..end { if a[i]>a[i+1] { a.swap(i,i+1); swapped=true; } } if !swapped { break; } } }
pub fn insertion_sort<T: Ord>(a: &mut [T]) { for i in 1..a.len() { let mut j=i; while j>0 && a[j]<a[j-1] { a.swap(j,j-1); j-=1; } } }
pub fn selection_sort<T: Ord>(a: &mut [T]) { for i in 0..a.len() { let mut m=i; for j in i+1..a.len() { if a[j]<a[m] { m=j; } } if m!=i { a.swap(i,m); } } }
pub fn shell_sort<T: Ord>(a: &mut [T]) { let mut gap=a.len()/2; while gap>0 { for i in gap..a.len() { let mut j=i; while j>=gap && a[j]<a[j-gap] { a.swap(j,j-gap); j-=gap; } } gap/=2; } }
pub fn merge_sort<T: Ord+Clone>(a:&[T])->Vec<T>{if a.len()<2{return a.to_vec()}let m=a.len()/2;merge(&merge_sort(&a[..m]),&merge_sort(&a[m..]))}
fn merge<T:Ord+Clone>(l:&[T],r:&[T])->Vec<T>{let mut o=Vec::with_capacity(l.len()+r.len());let(mut i,mut j)=(0,0);while i<l.len()&&j<r.len(){if l[i]<=r[j]{o.push(l[i].clone());i+=1}else{o.push(r[j].clone());j+=1}}o.extend_from_slice(&l[i..]);o.extend_from_slice(&r[j..]);o}
pub fn quick_sort<T:Ord>(a:&mut[T]){if a.len()<2{return}let p=partition(a);let(l,r)=a.split_at_mut(p);quick_sort(l);quick_sort(&mut r[1..]);}
fn partition<T:Ord>(a:&mut[T])->usize{let p=a.len()-1;let mut i=0;for j in 0..p{if a[j]<=a[p]{a.swap(i,j);i+=1}}a.swap(i,p);i}
pub fn heap_sort<T:Ord>(a:&mut[T]){fn sift<T:Ord>(a:&mut[T],mut r:usize,n:usize){loop{let l=2*r+1;if l>=n{return}let mut c=l;if l+1<n&&a[l]<a[l+1]{c=l+1}if a[r]>=a[c]{return}a.swap(r,c);r=c}}let n=a.len();if n<2{return}for i in (0..n/2).rev(){sift(a,i,n)}for end in (1..n).rev(){a.swap(0,end);sift(a,0,end)}}
pub fn counting_sort(a:&mut [usize]){if a.is_empty(){return}let max=*a.iter().max().unwrap();let mut c=vec![0usize;max+1];for &x in a.iter(){c[x]+=1}let mut k=0;for (x,n) in c.into_iter().enumerate(){for _ in 0..n{a[k]=x;k+=1}}}
pub fn radix_sort(a:&mut [u64]){if a.len()<2{return}let mut buf=vec![0u64;a.len()];for shift in (0..64).step_by(8){let mut count=[0usize;256];for &x in a.iter(){count[((x>>shift)&255)as usize]+=1}let mut sum=0;for c in count.iter_mut(){let n=*c;*c=sum;sum+=n}for &x in a.iter(){let i=((x>>shift)&255)as usize;buf[count[i]]=x;count[i]+=1}a.copy_from_slice(&buf)}}
pub fn stable_sort<T:Ord>(a:&mut[T]){a.sort()}
pub fn unstable_sort<T:Ord>(a:&mut[T]){a.sort_unstable()}
