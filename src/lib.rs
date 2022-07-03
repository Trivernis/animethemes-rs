//! This crate provides a client for the AnimeThemes.moe api.
//!
//! ## Search
//! ```
//! # use animethemes_rs::error::ApiResult;
//! use animethemes_rs::client::AnimeThemesClient;
//! use animethemes_rs::includes::SearchIncludes;
//!
//! # async fn a() -> ApiResult<()> {
//! let client = AnimeThemesClient::default();
//! let response = client.search("Vivy", &[], SearchIncludes::default()).await?;
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
pub mod includes;
pub mod models;
