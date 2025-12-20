```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- `use` キーワードで、パスを一度スコープに持ち込むことで、その後のコードでそのパスを繰り返し書く必要がなくなる

## 慣例の従った`use`パスを作る

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- `std::collections::HashMap` のように、慣例に従ったパスを使うことで、コードの可読性が向上する
- Rust コミュニティでは、標準ライブラリの型を使う際に、`use` キーワードでインポートすることが一般的である

## 新しい名前を`as`キーワードで与える

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

- `as` キーワードを使うことで、同じ名前の型や関数が衝突するのを防ぐことができる

## `pub use`で名前を再公開する

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- `pub use` を使うことで、モジュールの外部に名前を再公開できる

## 外部のパッケージを使う

```toml
rand = "0.8.5"
```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

- `Cargo.toml` に依存関係を追加し、`use` キーワードで外部クレートの機能をインポートできる

## 巨大な`use`のリストをネストしたパスを使って整理する

```rust
// --snip--
// （略）
use std::cmp::Ordering;
use std::io;
// --snip--
// （略）
```

```rust
// --snip--
// （略）
use std::{cmp::Ordering, io};
// --snip--
// （略）
```

- 複数のアイテムをインポートする際に、ネストしたパスを使うことでコードを整理できる
- `std::` の後に中括弧 `{}` を使い、その中にインポートしたいアイテムをカンマで区切って列挙する

## `glob`演算子

```rust
use std::collections::*;
```

- `*` を使うことで、そのモジュール内のすべての公開アイテムをインポートできる
- ただし、どのアイテムがスコープに入るかが明確でなくなるため、乱用は避けるべきである

$\uparrow$ まぁよくわからんよね

---

$\downarrow$ AI さんに教えてもらう

## 住所と「呼び名」

毎回「東京都千代田区永田町 1 丁目...」にある「国会議事堂」に手紙を書くとするが、毎回住所をすべて書くのは面倒くさい

- **フルパス**: `東京都::千代田区::永田町::国会議事堂`
- **`use`を使う（ショートカット）**: `use 東京都::千代田区::永田町::国会議事堂;`（これ以上、`国会議事堂`とだけ書けばよい）
