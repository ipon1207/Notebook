## Cargo

CargoはRustのビルドシステムとパッケージマネージャーを兼ねる
コードのビルドや依存(dependencies)のライブラリのダウンロード、そのライブラリのビルドなどを行う
Cargoはプロジェクトが大規模化したときにその真価を発揮する

## プロジェクトの作成

```cmd
> cargo new hello_cargo
> cd hello_cargo
```

- 以上のコマンドを実行すると `Cargo.toml` と `.gitignore` 、`src/main.rs` が作成される
- Gitリポジトリの初期化も行われる

**Cargo.toml**
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- TOML(Tom's Obvious, Minimal Language)形式で、Cargoの設定フォーマット
- `[package]` はセクションヘッダーで、パッケージ情報を設定する
- `[dependencies]` は、プロジェクトの依存を列挙する（パッケージのことを**クレート**と呼ぶ）

**src/main.rs**
```rust
fn main() {
	println!("Hello, world!");
}
```

- Cargoは `src` 以下にソースファイルがあることを想定している
- 最上位のディレクトリには、READMEやライセンス情報、設定ファイルなどを置く

## Cargoプロジェクトをビルドして実行