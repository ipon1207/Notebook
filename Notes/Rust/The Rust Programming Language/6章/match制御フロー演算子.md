```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- `match` アームごとに異なる列挙子をパターンマッチング
- 各アームに紐づけられるコードは式でなければならない

## 値に束縛されるパターン

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // 他の州もここに追加
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

- `Quarter(UsState)` のように、列挙子に関連付けられたデータをパターンマッチングで取得可能
- `state` 変数に関連付けられたデータが束縛され、アーム内で使用できる

## `Option<T>` とのマッチ

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- `Option<T>` 列挙型を使用して、値が存在するかどうかを表現
- `Some(i)` のパターンマッチングで値を取得し、操作可能

## `match` の性質

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

- `match` は網羅的でなければならないため、上記のコードはコンパイルエラーになる
- `None` のケースがカバーされていないため、すべての可能なケースを扱う必要がある

## `_` プレースホルダー

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

- `_` はワイルドカードパターンとして機能し、他のすべての値にマッチ
- 具体的な値にマッチしない場合のデフォルトケースとして使用
- `()` は何もしないことを意味するユニット値

参考(https://doc.rust-jp.rs/book-ja/ch06-02-match.html)
