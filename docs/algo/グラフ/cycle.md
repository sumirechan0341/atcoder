# 閉路

## 定義
始点からスタートして、同じ頂点を使わないで辺をたどっていき、始点まで戻るパスを閉路という。

## 閉路検出
### SCC
ここにコードを貼るかlibraryを作ってそのリンクを貼るか
コード+説明を書いてlibraryにはコードのみが見やすそう

## 性質・テクニック
- 閉路検出のときに、閉路の部分のみ出力したいということがたびたびある。
閉路を検出できた頂点を視点として、もう1周すれば閉路のみを切り出すことができる。

  例題
  - [ABC311 C問題](https://atcoder.jp/contests/abc311/tasks/abc311_c)
    
    [解答例](/problems/abc311/src/c.rs)
