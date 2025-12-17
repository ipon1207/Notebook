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
