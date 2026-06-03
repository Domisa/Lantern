# Lantern

A production-style REST API built in Rust — task management backend with a full container and Kubernetes deployment pipeline.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-orange?style=flat)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=flat&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white)
![Kubernetes](https://img.shields.io/badge/Kubernetes-326CE5?style=flat&logo=kubernetes&logoColor=white)
![CI](https://img.shields.io/badge/CI-GitHub_Actions-2088FF?style=flat&logo=githubactions&logoColor=white)

---

## Overview

Lantern is a task management REST API written in Rust using Axum and SQLx, backed by PostgreSQL. The project is containerized with a two-stage Docker build and deployed to Kubernetes, with a GitHub Actions CI pipeline for automated builds.

It was built as a portfolio project to demonstrate production-minded backend development: clean module structure, compile-time verified queries, secrets management, and infrastructure-as-code from local dev through container orchestration.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (stable) |
| Web framework | Axum |
| Database | PostgreSQL |
| ORM / query layer | SQLx (compile-time checked, offline cache) |
| Serialization | Serde / serde_json |
| Containerization | Docker (multi-stage build) |
| Orchestration | Kubernetes |
| Local dev | Docker Compose |
| CI/CD | GitHub Actions |

---

## Features

- Full CRUD for tasks (create, read, update, delete)
- PostgreSQL-backed persistence with SQLx migrations
- Compile-time query verification via `cargo sqlx prepare` offline cache
- Two-stage Docker build for minimal production image size
- Docker Compose setup for fast local development
- Kubernetes manifests for in-cluster Postgres and app deployment
- Credentials managed via Kubernetes Secrets
- GitHub Actions CI pipeline with repository secrets

---

## Project Structure

```
lantern/
├── src/
│   ├── main.rs           # Server setup, router, DB pool initialization
│   ├── routes/
│   │   ├── mod.rs        # Route registration
│   │   └── tasks.rs      # Task CRUD handlers
│   └── models/           # Shared types and DB structs
├── migrations/           # SQLx migration files
├── k8s/
│   ├── deployment.yaml          # Lantern app deployment
│   ├── service.yaml             # Lantern service
│   ├── postgres-deployment.yaml # In-cluster Postgres
│   ├── postgres-service.yaml    # Postgres service
│   └── secret.yaml              # DB credentials (gitignored)
├── .sqlx/                # Offline query cache (cargo sqlx prepare)
├── Dockerfile            # Two-stage build
├── docker-compose.yml    # Local dev environment
└── .github/
    └── workflows/
        └── ci.yml        # GitHub Actions pipeline
```

---

## Getting Started

### Prerequisites

- Rust (stable)
- Docker and Docker Compose
- `sqlx-cli` — `cargo install sqlx-cli`

### Local development (Docker Compose)

```bash
# Clone the repo
git clone https://github.com/your-username/lantern.git
cd lantern

# Start Postgres and the API
docker compose up

# The API will be available at http://localhost:3000
```

### Running migrations manually

```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432/lantern
sqlx migrate run
```

### Building with offline query cache

```bash
cargo sqlx prepare   # generates .sqlx/ cache
cargo build --release
```

---

## API Reference

Base URL: `http://localhost:3000`

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/tasks` | List all tasks |
| `GET` | `/tasks/:id` | Get a task by ID |
| `POST` | `/tasks` | Create a new task |
| `PUT` | `/tasks/:id` | Update a task |
| `DELETE` | `/tasks/:id` | Delete a task |

### Example request

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Write documentation", "completed": false}'
```

### Example response

```json
{
  "id": 1,
  "title": "Write documentation",
  "completed": false,
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

## Kubernetes Deployment

The app deploys to a local cluster (minikube or kind) with Postgres running in-cluster.

```bash
# Apply secrets first (not committed — create your own)
kubectl apply -f k8s/secret.yaml

# Deploy Postgres
kubectl apply -f k8s/postgres-deployment.yaml
kubectl apply -f k8s/postgres-service.yaml

# Deploy Lantern
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Verify pods are running
kubectl get pods
```

The app connects to Postgres via the in-cluster service name as the `DATABASE_URL` host. Credentials are injected from a Kubernetes Secret.

---

## CI/CD

GitHub Actions runs on every push to `main`:

1. Checks out the repo
2. Installs the Rust toolchain
3. Restores the SQLx offline cache (`.sqlx/`)
4. Builds the project with `cargo build`
5. Runs tests with `cargo test`

Secrets (`DATABASE_URL`, etc.) are stored as GitHub repository secrets and never hardcoded.


## License

MIT
