## Rust でプログラムを書く

```Rust
fn main() {
	// 世界よ、こんにちは
	println!("Hello world");
}
```

- ファイル名は `main.rs` （**ファイル名に 2 単語以上使っているなら、アンダースコアで区切る**）

```cmd
> rustc main.rs
> .\main.exe
Hello, world!
```

## `Hello, world!` プログラムの詳細

```rust
fn main() {

}
```

- `main` 関数は特別で、常に全ての Rust プログラムにおいて最初に実行されるコード

```rust
    println!("Hello, world!");
```

- Rust のスタイルは、タブではなく、4 つのスペースでインデントする
- `println!` は[[Rustのマクロ]]を呼び出している（`!` がないと関数を呼び出すことになる）

## コンパイルと実行

> [!IMPORTANT]
> Rust は**AOT コンパイル(Ahead-Of-Time)**言語
> なので、コンパイル後の `exe` ファイルは誰でも実行できる

(https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)
