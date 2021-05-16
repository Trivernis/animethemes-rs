use crate::error::ApiResult;
use crate::models::{Anime, Artist, SearchResponse};
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;

pub static DEFAULT_API_ENDPOINT: &str = "https://staging.animethemes.moe/api";
pub static DEFAULT_VIDEO_ENDPOINT: &str = "https://animethemes.moe/video/";

#[derive(Clone, Debug)]
pub struct AnimeThemesClient {
    api_endpoint: String,
    video_endpoint: String,
    client: reqwest::Client,
}

impl Default for AnimeThemesClient {
    fn default() -> Self {
        Self {
            api_endpoint: DEFAULT_API_ENDPOINT.to_string(),
            video_endpoint: DEFAULT_VIDEO_ENDPOINT.to_string(),
            client: reqwest::Client::default(),
        }
    }
}

impl AnimeThemesClient {
    /// Creates a new AnimeThemesClient
    pub fn new(api_endpoint: &str, video_endpoint: &str) -> Self {
        Self {
            api_endpoint: api_endpoint.to_string(),
            video_endpoint: video_endpoint.to_string(),
            client: reqwest::Client::default(),
        }
    }

    /// Searches for all types provided by the api
    /// fields and include can be used to control what to include in the results
    /// if no fields are specified, all are returned
    /// This crate provides a client for the AnimeThemes.moe api.
    ///
    /// ```
    /// # use animethemes_rs::error::ApiResult;
    /// use animethemes_rs::client::AnimeThemesClient;
    ///
    /// # async fn a() -> ApiResult<()> {
    /// let client = AnimeThemesClient::default();
    /// let response = client.search("Attack on Titan", &[], &[]).await?;
    ///
    /// assert!(response.anime.is_some());
    /// assert!(response.songs.is_some());
    /// # Ok(()) }
    /// ```
    pub async fn search(
        &self,
        query: &str,
        fields: &[&str],
        include: &[&str],
    ) -> ApiResult<SearchResponse> {
        let mut query = vec![("q", query.to_string()), ("include", include.join(","))];

        if !fields.is_empty() {
            query.push(("fields[search]", fields.join(",")));
        }
        let mut response: HashMap<String, SearchResponse> =
            self.api_get("/search", &query[..]).await?.json().await?;

        Ok(response.remove("search").unwrap())
    }

    /// Returns an anime by a given slug string
    pub async fn anime(&self, slug: &str, include: &[&str]) -> ApiResult<Anime> {
        let mut response: HashMap<String, Anime> = self
            .api_get(
                format!("/anime/{}", slug).as_str(),
                &[("include", include.join(","))],
            )
            .await?
            .json()
            .await?;

        Ok(response.remove("anime").unwrap())
    }

    /// Returns an artist by a given slug string
    pub async fn artist(&self, slug: &str, include: &[&str]) -> ApiResult<Artist> {
        let mut response: HashMap<String, Artist> = self
            .api_get(
                format!("/artist/{}", slug).as_str(),
                &[("include", include.join(","))],
            )
            .await?
            .json()
            .await?;

        Ok(response.remove("artist").unwrap())
    }

    /// Posts a get request to the API endpoint
    async fn api_get<T: Serialize + ?Sized>(&self, path: &str, query: &T) -> ApiResult<Response> {
        let response = self
            .client
            .get(format!("{}{}", self.api_endpoint, path))
            .query(query)
            .send()
            .await?;

        Ok(response)
    }
}
