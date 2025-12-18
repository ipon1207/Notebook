```rust
let some_u8_value = Some(0u8);
if let some(3) = some_u8_value {
    println!("three");
}
```

- `if let` 構文を使用して、特定のパターンにマッチする場合にのみコードを実行
- `if let` を使用することで、`match` よりも簡潔に書ける場合がある

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

- `else` ブロックを追加して、パターンにマッチしなかった場合の処理も記述可能

参考(https://doc.rust-jp.rs/book-ja/ch06-03-if-let.html)
