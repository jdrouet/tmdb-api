# Rust client for The Movie DB API

This is yet another client for TMDB, but it supports async functions.

## Installing

```bash
cargo add tmdb-api
```

## Usage

```rust,no_run
use tmdb_api::client::Client;
use tmdb_api::client::reqwest::reqwest::Client as ReqwestClient;

#[tokio::main]
async fn main() {
    let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
    let client = Client::<ReqwestClient>::new(secret);
    let res = client.search_tvshows("simpsons", &Default::default()).await.unwrap();
    let item = res.results.first().unwrap();
    println!("TVShow found: {}", item.inner.name);
}

```

## Features

## Running the tests

```bash
cargo test
```

If you want to run some integration tests, just export a `TMDB_TOKEN_V3` environment variable and run

```bash
cargo test --features integration
```
