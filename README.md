# Rusty Activity Pub

Implementing an ActivityPub server in Rust

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

**Note**: Please populate the `.env` file with `DATABASE_URL=postgres://user:password@127.0.0.1:port/rusty_activity_pub`.

<details>
    <summary>Instructions to embed Migrations into application binary</summary>
  
    On startup, after creating your database connection or pool, add:

    ```rust
    sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
    ```

    Note that the compiler won't pick up new migrations if no Rust source files have changed.
    You can create a Cargo build script to work around this with `sqlx migrate build-script`.

    See: [link](https://docs.rs/sqlx/0.5/sqlx/macro.migrate.html)

</details>

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

