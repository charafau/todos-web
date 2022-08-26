## Nothing interesting here, just rust playground 🦀 🐚

To run connection string must be added:

```
echo DATABASE_URL=postgres://username:password@localhost/database_name > .env
```

Just run:
```
cargo run
```

Run and watch for changes:

```
cargo watch -x run
```

To run migrations `diesel_cli` needs to be installed (example for postgres)

```
cargo install diesel_cli  --no-default-features --features postgres
```
