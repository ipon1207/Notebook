## Rustでプログラムを書く

```Rust
fn main() {
	// 世界よ、こんにちは
	println!("Hello world");
}
```

- ファイル名は `main.rs` （**ファイル名に2単語以上使っているなら、アンダースコアで区切る**）

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

- `main` 関数は特別で、常に全てのRustプログラムにおいて最初に実行されるコード