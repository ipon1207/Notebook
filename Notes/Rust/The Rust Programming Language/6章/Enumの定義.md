## Enum の値

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- `enum ~` で列挙型を定義
- `V4` と `V6` は `IpAddrKind` 型の異なる列挙子（バリアント）
- `IpAddrKind::V4` のようにして列挙子を参照

```rust
enum IpAddrKind {
    V4,
    V6,
}

let home = IpAddrKind::V4(String::from("127.0.0.1"));
let loopback = IpAddrKind::V6(String::from("::1"));
```

- 列挙子にデータを関連付けることも可能
- `V4` と `V6` はそれぞれ `String` 型のデータを持つ

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

- 列挙子に異なる型や数のデータを持たせることも可能

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッドの実装
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

- 列挙型にメソッドを実装することも可能
- `impl` ブロック内でメソッドを定義し、列挙型のインスタンスに対して呼び出せる

## `Option enum`

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- Rust 標準ライブラリに定義されている汎用的な列挙型
- `None` は値が存在しないことを表し、`Some(T)` は値が存在することを表す
- `Option<T>` を使うことで、ヌルポインタを避け、安全に値の有無を扱える

参考(https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html)
