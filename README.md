# Quests Tracker

A backend web service written in Rust for managing quests, adventurers, and guild commanders. Built with clean architecture principles.

---

## 🚀 Tech Stack

- **Core Framework:** [Rust](https://www.rust-lang.org/) with [Axum](https://github.com/tokio-rs/axum) (Web framework) & [Tokio](https://tokio.rs/) (Asynchronous runtime)
- **Database ORM:** [Diesel](https://diesel.rs/) with PostgreSQL
- **Containerization:** Docker & Docker Compose
- **CI/CD:** GitHub Actions (configured with self-hosted runner)

---

## 🛠️ Environment Variables

Create a `.env` file in the root directory and configure the following variables:

```ini
STAGE=Local

SERVER_PORT=8080
SERVER_BODY_LIMIT=10 # MB
SERVER_TIMEOUT=30    # seconds

DATABASE_URL=postgres://username:password@localhost:5432/quests_tracker

JWT_ADVENTURER_SECRET=your_adventurer_secret_key
JWT_ADVENTURER_REFRESH_SECRET=your_adventurer_refresh_secret_key
JWT_GUILD_COMMANDER_SECRET=your_commander_secret_key
JWT_GUILD_COMMANDER_REFRESH_SECRET=your_commander_refresh_secret_key
```

---

## 💻 Local Development

### Prerequisites

- [Rust (1.86+)](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/) & Docker Compose
- [Diesel CLI](https://diesel.rs/guides/getting-started) (optional, for migrations)

### 1. Database Setup

Spin up the PostgreSQL database container:

```bash
docker compose up -d postgres
```

### 2. Run Database Migrations

```bash
diesel migration run
```

### 3. Run the Application

To start the server in development mode:

```bash
cargo run
```

The server will start listening on the port configured in your `SERVER_PORT` environment variable (default: `8080`).

---

## 🐳 Docker Deployment

To run the entire stack (web application + PostgreSQL database) using Docker Compose:

```bash
docker compose up --build -d
```

---

## 🧪 CI/CD Pipeline

The project uses a GitHub Actions workflow (`.github/workflows/main.yml`) executing on a **self-hosted runner**:

- **Linting & Formatting:** Checks code formatting with `cargo fmt` and lints with `cargo clippy`.
- **Testing:** Runs unit and integration tests with `cargo test`.
- **Building:** Compiles the production binary with `cargo build --release`.
- **Dockerizing:** Builds the production Docker image and pushes it to **GitHub Container Registry (GHCR)**:
  - Repository Image: `ghcr.io/<owner>/quests-tracker:latest` and tagged with the commit SHA.

---

## 💡 Credits & Acknowledgements

This project was developed for educational purposes to learn Backend development with Rust, based on the original repository:
- **Original Repository:** [Rayato159/quests-tracker](https://github.com/Rayato159/quests-tracker)

