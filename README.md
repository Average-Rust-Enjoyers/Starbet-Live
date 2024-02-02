# Starbet-Live

## Overview
The goal is to create a betting system for esports. Esports have different caterogies (games) in each of which there are individual matches on certain days and times. There are odds for each match, which can change during the match. You can bet only if the match is running. The bets affect the odds of the games. When the odds change or a new bet is posted, the change is reflected realtime in all connected client applications.

## Getting Started

### Prerequisites

- Ensure you have Docker Compose installed. If not, you can download it from [Docker Desktop](https://www.docker.com/products/docker-desktop).

### Self host Setup

1. **Environment Variables**: Copy the sample environment file. Change passwords and optionally add your own API key. Process to get your own API key can be found here: [Cloudbet API Key](https://www.cloudbet.com/api/)

    ```bash
    cp .env.example .env
    nano .env
    ```

2. **Docker Compose**: Start the Docker containers.

    ```bash
    docker compose up -d
    ```

3. **Connect**: Open your browser and paste the following url.

    `http://0.0.0.0:6969/`

### Hosted instance
Or alternatively you can connect to deployed instance at: `http://130.61.53.56:6969/` (there is no guarantee that it will work)

## Technologies Used

- **Rust**: The core programming language used for this project.
- **Axum**: A Rust framework for building web services.
- **Tokio**: An asynchronous runtime for Rust.
- **SQLx**: A Rust crate for compile-time checked SQL queries.
- **Serde**: A Rust crate for serializing and deserializing data structures.
- **Cynic**: A Rust GraphQL query builder & data mapper.
- **Askama**: A Rust template rendering engine.