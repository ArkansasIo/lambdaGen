pub fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{let r=a%b;a=b;b=r}a}
pub fn lcm(a:u64,b:u64)->u64{if a==0||b==0{0}else{a/gcd(a,b)*b}}
pub fn isqrt(n:u64)->u64{let(mut lo,mut hi)=(0,n.min(1<<32)+1);while lo+1<hi{let m=lo+(hi-lo)/2;if m<=n/m{lo=m}else{hi=m}}lo}
pub fn integer_pow(mut b:u64,mut e:u32)->u64{let mut r=1;while e>0{if e&1==1{r=r.saturating_mul(b)}e>>=1;if e>0{b=b.saturating_mul(b)}}r}
pub fn sieve(limit:usize)->Vec<usize>{if limit<2{return Vec::new()}let mut p=vec![true;limit+1];p[0]=false;p[1]=false;let mut i=2;while i<=limit/i{if p[i]{let mut j=i*i;while j<=limit{p[j]=false;j+=i}}i+=1}p.iter().enumerate().filter_map(|(i,&v)|v.then_some(i)).collect()}
pub fn fibonacci(n:u64)->u64{fn f(n:u64)->(u64,u64){if n==0{return(0,1)}let(a,b)=f(n/2);let c=a.saturating_mul(b.saturating_mul(2).saturating_sub(a));let d=a.saturating_mul(a).saturating_add(b.saturating_mul(b));if n%2==0{(c,d)}else{(d,c.saturating_add(d))}}f(n).0}
pub fn collatz_steps(mut n:u64,limit:u64)->Option<u64>{if n==0{return None}let mut s=0;while n!=1&&s<limit{n=if n%2==0{n/2}else{n.checked_mul(3)?.checked_add(1)?};s+=1}if n==1{Some(s)}else{None}}
