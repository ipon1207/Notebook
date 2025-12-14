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

```cmd
> cargo build
Compiling hello_cargo v0.1.0 (C:\Users\shuji\Projects\Rust\hello_cargo)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
```

- コマンドを実行することで `target/debug/プロジェクト名.exe` として作成される
- `Cargo.lock` というプロジェクト内の依存関係のバージョンを記録するファイルを生成する
- `cargo run` を使うと生成した `exe` ファイルの実行までしてくれる

```cmd
> cargo check
Checking hello_cargo v0.1.0 (C:\Users\shuji\Projects\Rust\hello_cargo)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
```

- `cargo check` を使うことでコンパイル可能かを確認できる
- 基本的には `cargo check` でコンパイル可能なら `cargo build` をする流れ

> [!IMPORTANT]
> ### まとめ
> - `cargo new`: プロジェクトを作成
> - `cargo build`: プロジェクトをビルド
> - `cargo run`: ビルドと実行を一括で行う
> - `cargo check`: バイナリを生成せずにプロジェクトをビルドする

> [!NOTE]
> ### リリース用のビルド
> `cargo build --release` を使うことで、最適化した状態でコンパイルできる
> これによって実行ファイルは `target/release` に生成される
> コンパイルには時間がかかるため、最終的なプログラムに対して利用することが基本

(https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)