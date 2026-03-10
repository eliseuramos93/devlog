# DevLog

A blog platform built in Rust as a learning project, covering the core topics from [roadmap.sh/backend](https://roadmap.sh/backend).

The goal is not to ship a product — it is to understand how backend systems work at a deeper level: HTTP, databases, authentication, async processing, observability, and distributed systems concepts. Each phase introduces new concepts through the Rust compiler, which is treated as a teacher.

## Stack

- **Rust** — Axum (HTTP), Tokio (async runtime), SQLx (typed SQL), Serde (serialization)
- **PostgreSQL** — primary database
- **Redis** — caching and session storage
- **RabbitMQ** — async job processing
- **Elasticsearch** — full-text search
- **Prometheus + Grafana** — metrics and observability

## Project structure

```
crates/
├── api/     # HTTP server, routes, handlers
├── domain/  # Business logic and entities
├── infra/   # Database, Redis, RabbitMQ clients
└── worker/  # Background job processing
```

## Running locally

This project runs entirely inside a devcontainer. You need Docker and VSCode with the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.

1. Clone the repository
2. Open in VSCode
3. `F1` → `Dev Containers: Reopen in Container`
4. Copy `.env.example` to `.env`
5. `cargo build`
