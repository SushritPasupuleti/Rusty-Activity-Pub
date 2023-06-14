# Rusty Activity Pub

Implementing an ActivityPub server in Rust.

**Note**: This is just an implementation that puts together all necessary logic to build an `ActivityPub` server. This includes things like authentication, hashing, and other necessary mechanisms.

## Getting Started

Running the server:

```
cd server
cargo run
```

Setting up the database:

```
cd server
sqlx database create
sqlx migrate run
```

Create new migration:

```
cd server
sqlx migrate add <migration_name>
```

### Development

Auto-reloading server:

```
cargo install cargo-watch
```

```
cd server
cargo watch -q -c -w src/ -x run
```

## Docs

### Folder Structure

```
- server/
    - cargo.toml
    - cargo.lock
    - src/
        - ActivityPub.rs
        - handlers.rs
        - main.rs
        - middleware.rs
        - models.rs
        - routes.rs
        - schema.rs
        - utils.rs
        - ActivityPub/
            ...ActivityPub (main logic)
        - handlers/
            ...handlers
        - middleware/
            ...middleware
        - models/
            ...models (main table struct and common related forms of them)
        - routes/
            ...routes
        - schema/
            ...schema (structs like those used for results)
        - utils/
            ...utils (common functions and structs used in multiple places)
```
