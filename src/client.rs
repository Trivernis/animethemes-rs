use crate::error::ApiResult;
use crate::includes::*;
use crate::models::{
    Anime, AnimeSynonym, Artist, Image, Resource, SearchResponse, Series, Song, Theme, ThemeEntry,
    Video,
};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub static DEFAULT_API_ENDPOINT: &str = "https://api.animethemes.moe";
pub static DEFAULT_VIDEO_ENDPOINT: &str = "https://animethemes.moe/video/";

#[derive(Clone, Debug)]
pub struct AnimeThemesClient {
    api_endpoint: String,
    pub video_endpoint: String,
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
    /// use animethemes_rs::includes::SearchIncludes;
    ///
    /// # async fn a() -> ApiResult<()> {
    /// let client = AnimeThemesClient::default();
    /// let response = client.search("Attack on Titan", &[], SearchIncludes::default()).await?;
    ///
    /// assert!(response.anime.is_some());
    /// assert!(response.songs.is_some());
    /// # Ok(()) }
    /// ```
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn search(
        &self,
        query: &str,
        fields: &[&str],
        include: SearchIncludes,
    ) -> ApiResult<SearchResponse> {
        let mut query = vec![("q".to_string(), query.to_string())];
        query.append(&mut include.indo_includes());

        if !fields.is_empty() {
            query.push(("fields[search]".to_string(), fields.join(",")));
        }
        let mut response: HashMap<String, SearchResponse> =
            self.api_get("/search", &query[..]).await?.json().await?;

        Ok(response.remove("search").unwrap())
    }

    /// Returns an anime by a given slug string
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn anime(&self, slug: &str, include: AnimeInclude) -> ApiResult<Anime> {
        self.entry_by_id_with_include("anime", slug, include.includes())
            .await
    }

    /// Returns an artist by a given slug string
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn artist(&self, slug: &str, include: ArtistInclude) -> ApiResult<Artist> {
        self.entry_by_id_with_include("artist", slug, include.includes())
            .await
    }

    /// Returns an entry by a given id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn entry(&self, id: u32, include: ThemeEntryInclude) -> ApiResult<ThemeEntry> {
        self.entry_by_id_with_include("animethemeentry", id, include.includes())
            .await
    }

    /// Returns an image by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn image(&self, id: u32, include: ImageInclude) -> ApiResult<Image> {
        self.entry_by_id_with_include("image", id, include.includes())
            .await
    }

    /// Returns a resource by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn resource(&self, id: u32, include: ResourceInclude) -> ApiResult<Resource> {
        self.entry_by_id_with_include("resource", id, include.includes())
            .await
    }

    /// Returns a series by slug
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn series(&self, slug: &str, include: SeriesInclude) -> ApiResult<Series> {
        self.entry_by_id_with_include("series", slug, include.includes())
            .await
    }

    /// Returns a song by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn song(&self, id: u32, include: SongInclude) -> ApiResult<Song> {
        self.entry_by_id_with_include("song", id, include.includes())
            .await
    }

    /// Returns a synonym by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn synonym(&self, id: u32, include: SynonymInclude) -> ApiResult<AnimeSynonym> {
        self.entry_by_id_with_include("animesynonym", id, include.includes())
            .await
    }

    /// Returns a theme by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn theme(&self, id: u32, include: ThemeInclude) -> ApiResult<Theme> {
        self.entry_by_id_with_include("animetheme", id, include.includes())
            .await
    }

    /// Returns a video by basename
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn video(&self, basename: &str, include: VideoInclude) -> ApiResult<Video> {
        self.entry_by_id_with_include("video", basename, include.includes())
            .await
    }

    /// Generic endpoint with the format /<endpoint>/<id> returning the type on the json field <endpoint>
    #[tracing::instrument(level = "debug", skip(self))]
    async fn entry_by_id_with_include<T: DeserializeOwned, I: Display + Debug>(
        &self,
        endpoint: &str,
        id: I,
        include: Vec<String>,
    ) -> ApiResult<T> {
        let mut response: HashMap<String, T> = self
            .api_get(
                format!("/{}/{}", endpoint, id).as_str(),
                &[("include", include.join(","))],
            )
            .await?
            .json()
            .await?;

        Ok(response.remove(endpoint).unwrap())
    }

    /// Starts a get request to the API endpoint
    #[tracing::instrument(level = "trace", skip(self))]
    async fn api_get<T: Serialize + Debug + ?Sized>(
        &self,
        path: &str,
        query: &T,
    ) -> ApiResult<Response> {
        let response = self
            .client
            .get(format!("{}{}", self.api_endpoint, path))
            .query(query)
            .send()
            .await?;

        Ok(response)
    }
}
