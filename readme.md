# Rust client for The Movie DB API

This is yet another client for TMDB but it supports async functions.

## Running the tests

```bash
cargo test
```

If you want to run some integration tests, just export a `TMDB_TOKEN_V3` environemnt variable and run

```bash
cargo test --features integration
```