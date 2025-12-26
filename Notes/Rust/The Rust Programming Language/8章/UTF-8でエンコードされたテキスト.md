## 文字列の定義

Rust には、言語の核として 1 種類しか文字列型が存在しない
文字列スライス([[スライス型]])の`str`で、借用された形態`&str`として使われる

`String`型は、伸長可能、可変、所有権のある UTF-8 エンコードされた文字列型

「文字列」とは、`String`と文字列スライスの`&str`のことを指す

## 新規文字列を生成する

```rust
let mut s = String::new();
```

- `String::new`関数は空の文字列を生成する

```rust
let data = "initial contents";
let s = data.to_string();

let s = "initial contents".to_string();
```

- `to_string`メソッドは、文字列スライスから`String`を生成する

```rust
let s = String::from("initial contents");
```

- `String::from`関数も、文字列スライスから`String`を生成する

## 文字列を更新する

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

- `push_str`メソッドは、文字列スライスを`String`の末尾に追加する
- `push_str`は借用した文字列スライスを取るため、元のスライスはその後も使用可能

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はもう使えない
```

- `+`演算子は、左辺の`String`を右辺の`&str`に追加する
- `+`演算子は、`String`の所有権を左辺から奪うため、左辺の`String`はその後使用できない
- 右辺は借用されるため、元の`String`はその後も使用可能

## 文字列に添え字アクセスする

Rust では、文字列に添え字アクセスはできない
文字列は UTF-8 エンコードされており、各文字が異なるバイト数を持つため

## 文字列を走査するメソッド群

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

- `chars`メソッドは、文字列を Unicode スカラー値のイテレータとして走査する

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

- `bytes`メソッドは、文字列をバイトのイテレータとして走査する

参考(https://doc.rust-jp.rs/book-ja/ch08-02-strings.html)
