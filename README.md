# Actix-Web v4.9.0

Easily build and deploy applications with this Actix-web custom template.

---

## Installation

1. **Install `cargo-watch`**

To enable live-reloading during development, install `cargo-watch` using the following command:
```bash
   cargo install cargo-watch --locked
```

---

## Development Setup

To start developing, use the following steps:

1. **Run the development server**

Use `cargo-watch` to start the development server with live-reloading:
```bash
   cargo watch -x run
```
   This will monitor file changes and automatically restart the application.

---

## Build and Run with Docker

1. **Start services using Docker Compose**

Run the following command to start the database and the Actix-web application:
```bash
   docker compose up actix-db actix-app
```

---

## Resources

Before starting, ensure you have the following installed on your system:

- Actix-Web: [https://actix.rs](https://actix.rs)
- Rust: [https://www.rust-lang.org](https://www.rust-lang.org)
- Docker: [https://docker.com](https://docker.com)
