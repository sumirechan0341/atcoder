# 文字列

## 回文判定
sをreverseして元のsと一致するか見る。
```rust
fn isPalindrome(s: Vec<char>) -> bool {
  let mut rev_s = s.clone();
  rev_s.reverse();
  return s == rev_s;
}
```

もしくは文字列の半分の地点まで両端から見ていく。
```rust
fn isPalindrome(s: Vec<char>) -> bool {
  for i in 0..n/2 {
    if s[i] != s[n-i-1] {
      return false;
    }
  }
  return true;
}
```

## 性質・テクニック
- 回文は反転しても同じ文字列になる
- 何かのオーダーが遅かった話
- sliceは高速
- sliceを使って連結する方法