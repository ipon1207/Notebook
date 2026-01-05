アプリケーションの開発において、機能の実現にDockerを利用する場合は、開発にDockerを利用しようとすると入れ子構造になってしまうのでは？

```Text
[ あなたのPC (Mac / Windows / Linux) ]
    │
    ├── [ Docker Daemon ] <─────────┐
    │                               │
    ├── [ docker-compose up ]       │
    │     ├── Next.js               │
    │     ├── PostgreSQL            │
    │     └── Rust API Server ──────┘
    │            │ 「ユーザーのコードが来たから実行して！」
    │            │  と、本体のDockerに命令を送る
    │            │
    │            ▼
    └── [ 使い捨てコンテナ (Rust User Code) ]
        「Hello World」を実行してすぐ消える
```

## 解説

通常、コンテナの中からは、外側のDocker（ホストのDocker）は見えないが、「**Dockerソケット**」というファイルを共有することで、コンテナの中から外側のDockerを操作できるようになる

`docer-compose.yml` で以下のように設定する

```YAML
services:
  rust-api:
	volumes:
	  - /var/run/docker.sock:/var/run/docker.sock
```