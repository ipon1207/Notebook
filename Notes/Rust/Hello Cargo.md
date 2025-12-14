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
- Gitリポジトリの初期化も行われる（