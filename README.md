# atcoder
atcoder

# 学んだもの一覧
- 二部探索するときはソートしてからする（もしくはHashSetとかHashMapを使えば、探索は自動で二部探索相当になる）
- 剰余をとるときは引き算に気を付ける。+p して %pを取ればOK
- (a + b - 1) / b
- 方向探索は[dx, dy]みたいに表現するのがよい
- itertoolsのizipマクロ便利っぽい
    - enumarateと違ってusizeじゃないものが欲しいときとかに使う
    - 普通のzipが欲しいときも使う
- second borrowがなんとかかんとかって言われたときは、**前方の変数**にcloneをつければよい
- Vec<(index, value)>みたいなものをソートしたときはsort_by_keyを使えばよい
sort_by_key(|p| p.1)みたいに使う
- メモ https://zenn.dev/nsarbosh/articles/4ef65d54c8b2d7
- sliceで分割した文字列をうまく結合する方法を知りたい vec![s1, s2].concat()みたいにしてるが…
vec<&char>になるつらみ
- 割り算を掛け算にして評価する方法覚えた 誤差とか厳しいとき
- 逆に掛け算を割り算にして評価する方法も覚えた オーバーフローしそうなとき