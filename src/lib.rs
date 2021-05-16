//! This crate provides a client for the AnimeThemes.moe api.
//!
//! ## Search
//! ```
//! # use animethemes_rs::error::ApiResult;
//! use animethemes_rs::client::AnimeThemesClient;
//!
//! # async fn a() -> ApiResult<()> {
//! let client = AnimeThemesClient::default();
//! let response = client.search("Vivy", &[], &[]).await?;
//!
//! assert!(response.anime.is_some());
//! assert!(response.videos.is_some());
//! # Ok(()) }
//! ```

#[cfg(test)]
mod tests;
mod utils;

pub mod client;
pub mod error;
pub mod models;
