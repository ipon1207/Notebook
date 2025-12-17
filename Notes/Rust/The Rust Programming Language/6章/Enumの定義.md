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
