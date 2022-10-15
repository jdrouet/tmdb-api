# Rust client for The Movie DB API

This is yet another client for TMDB but it supports async functions.

## Installing

```bash
cargo add tmdb-api
```

## Usage

```rust
use tmdb_api::tvshow::search::TVShowSearch;
use tmdb_api::prelude::Command;
use tmdb_api::Client;

#[tokio::main]
async fn main() {
    let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
    let client = Client::new(secret);
    let cmd = TVShowSearch::new("simpsons".into());

    let result = cmd.execute(&client).await.unwrap();
    let item = result.results.first().unwrap();
    println!("TVShow found: {}", item.inner.name);
}

```

## Running the tests

```bash
cargo test
```

If you want to run some integration tests, just export a `TMDB_TOKEN_V3` environemnt variable and run

```bash
cargo test --features integration
```