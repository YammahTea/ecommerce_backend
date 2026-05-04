## 🤝 Contributing

This project is **open for contributions**!  
Whether you’re an experienced Rust developer or just want to add something useful (bug fixes, new features, documentation, tests, etc.), feel free to contribute.

Just fork the repo, create a feature branch, and submit a **Pull Request**. Every contribution counts ❤️

## 🏗️ Project Status

This project is **still under active development**.  
New features and improvements are being added regularly.

## 🛒 About This Project

This repository serves as a **real-world example** of how a clean, scalable, and production-ready backend for an **e-commerce website** should be built using Rust. It demonstrates best practices in architecture, security, performance, and modern development workflows.
## 🛠️ Tech Stack

![Rust](https://img.shields.io/badge/Rust-1.9+-000000?style=flat&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.8.8-000000?style=flat&logo=rust&logoColor=orange)
![Docker](https://img.shields.io/badge/Docker-latest-2496ED?style=flat&logo=docker&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-18-336791?style=flat&logo=postgresql&logoColor=white)

- **Language**: Rust 🦀 (v1.9+)
- **Web Framework**: Axum (v0.8.8)
- **Database**: PostgreSQL (v18) with SQLx
- **Containerization**: Docker 🐳

## 🚀 Getting Started

Follow these steps to set up and run the project locally.

### Prerequisites


- **[Rust](https://www.rust-lang.org/)** 🦀 — version **1.9** or higher (recommended: latest stable)
- **[Docker](https://www.docker.com/)** 🐳 — required to run the PostgreSQL database
- **Cargo** (included with Rust)
- **Git**
- **sqlx-cli** — for running database migrations

```bash
# Install sqlx-cli (with postgres support)
cargo install sqlx-cli --no-default-features --features rustls,postgres
````

### 1. Clone the Repository

```bash
git clone https://github.com/YammahTea/ecommerce_backend.git
cd ecommerce_backend
```

#### 2. Configuration
Create a .env file in the project root and add the following variables:
```text
# Database Configuration
POSTGRES_USER=YOUR_TEST_NAME
POSTGRES_PASSWORD=YOUR_TEST_PASSWORD
POSTGRES_DB_NAME=YOUR_DB_TEST_NAME

# Connection URL (update with your actual credentials)
DATABASE_URL=postgres://YOUR_TEST_NAME:YOUR_TEST_PASSWORD@localhost:5499/YOUR_DB_TEST_NAME?sslmode=disable

# Connection Pool Configuration (integer values)
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=1

# Timeouts (in seconds)
DB_ACQUIRE_TIMEOUT=10
DB_IDLE_TIMEOUT=300

# JWT Settings
# Time is in HOURS
ACCESS_TOKEN_EXPIRE=24

# Password Hashing (do NOT change after initial setup) (any number between 4 and 31)
BCRYPT_COST=10
# recommended to set it at 10, the higher you set it, 
# the higher the time it will take to create the user 


# Secret key for JWT (change this to a strong random string in production)
JWT_SECRET=your_super_secret_jwt_key_here_change_in_production

# Use the one you want for your current needs
# recommended for daily dev — info from libs, debug from code
RUST_LOG='info,ecommerce_backend=debug'

# everything — very noisy
#RUST_LOG='debug'

# errors only
#RUST_LOG='error'
```

#### 3. Database
* **1- Open docker desktop and login**

* **2- Run docker database:**
```bash
docker compose up -d 
```
* **3- Run sqlx-cli (after you have installed it)**
```bash
sqlx migrate run
```

#### 4. Run the app

```bash
cargo run
```
For automatic reloading during development (recommended), install and use cargo-watch:
```text
cargo install cargo-watch
cargo watch -x run
```
