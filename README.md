# AnimeThemes Rust API

This crate provides an API Wrapper for [AnimeThemes.moe](https://animethemes.moe).

## Usage

```rust
use animethemes_rs::client::AnimeThemesClient;

let client = AnimeThemesClient::default();
let response = client.search("Vivy", &[], &[]).await?;

assert!(response.anime.is_some());
assert!(response.videos.is_some());
```

## License

Apache-2.0