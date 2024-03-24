# Dona app

### Description

This is a simple app that allows users to donate to a person of their choice.

### Dependencies

Backend:

- Rust +1.76
- PostgreSQL +16
- Redis +7
- SeaMigration +0.12

Frontend (Not yet started):

- Node.js +20
- pnpm +8
- Vue.js +3
- Astro +4

### Installation and running

1. Clone the repository
2. Copy the `.env.example` file to `.env` and fill in the required variables.
3. Start the PostgreSQL and Redis servers. With docket compose, you can run `docker-compose up -d`.
4. Install the migration dependencies with `cargo install sea-orm-cli`.
5. Run the migrations with `sea-orm-cli migrate up` inside the root directory. This will create the required tables in the database.
6. Run `cargo run` in the root directory.
7. The server should now be running on `127.0.0.1:8080` for the GraphiQL interface and `127.0.0.1:8080/graphql` for the GraphQL endpoint.
