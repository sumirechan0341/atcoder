# A
## 思考
1年がMか月あり、毎月D日ある。
m = d<sub>1</sub> * d<sub>2</sub>
を満たす組み合わせが何個あるか答える問題。
ただし、mは月の数、d<sub>1</sub>は日の1桁目、d<sub>2</sub>は日の2桁目で、
d<sub>1</sub>, d<sub>2</sub>ともに2以上でなければならない。

[1..m]の範囲で、満たす整数の組みを求めればよい。
```rust
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        m: i32,
        d: i32
    };
    let mut count = 0;
    for i in 1..=m {
        for j in 1..=d {
            if j/10 < 2 || j%10 < 2 {
                continue;
            }
            if i == j/10 * (j%10) {

                count += 1;
            }
        }
    }
    println!("{}", count);
}
```

# B
## 思考
長さNの数列A<sub>N</sub>が与えられて、それをK回繰り返した数列Bについて考える。
Bにおける転倒数を求める問題。

実際に例を書いてみるとわかりやすい。
A = 132, K = 3として考える。
132|132|132

最初のブロックに注目する。
このブロック内での転倒数は1つである。

次は、ブロックの外の数と比較してみる。
自分よりインデックスの大きい数は、自分より右側にあるブロックすべてを見ればよいことがわかる。
しかも、これらはブロック単位で周期性があるため、
(注目しているブロックよりも右側にあるブロックの数) * (ブロック内で自分より大きかった要素の数)
で計算が可能である。

s[i]を同一ブロック内で、A[i]よりも大きい数字を持つ要素の数とする。
注目するブロックを1つずつ右にずらしていくと、累計カウントするブロックが1つずつ減っていくので、
\(\sum_{j=0}^{K-1}\) * \(\sum_{i=1}^{N} (s[i] * j)\) 
がブロック外との転倒数の総和である。
あとは各ブロックでブロック内での転倒数を求めればよい。
サイズが小さいのでシミュレーションすればよい。
全部足して答えになる。
```rust
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        an: [i64; n]
    };
    let p = 1000000007;
    let mut s = vec![0; n];
    let mut fst = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            } else if i < j {
                if an[i] > an[j] {
                    fst[i] += 1;
                    s[i] += 1;
                } 
            } else {
                if an[i] > an[j] {
                    s[i] += 1;
                }
            }
            
        }
    }
    let mut total = 0;
    for i in 0..n {
        total += (((((s[i]*(k-1))%p)*k)%p)*500000004)%p + (k*fst[i]%p);
        total %= p;
    }
    println!("{}", total);
}
```