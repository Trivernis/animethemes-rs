use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct EntryMetadata {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Anime {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub name: String,
    pub slug: String,
    pub year: u16,
    pub season: AnimeSeason,
    pub synopsis: String,
    #[serde(alias = "animesynonyms")]
    pub synonyms: Option<Vec<AnimeSynonym>>,
    #[serde(alias = "animethemes")]
    pub themes: Option<Vec<Theme>>,
    pub series: Option<Vec<Series>>,
    pub resource: Option<Vec<Resource>>,
    pub images: Option<Vec<Image>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum AnimeSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimeSynonym {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub text: String,
    pub anime: Option<Anime>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    #[serde(alias = "type")]
    pub theme_type: ThemeType,
    #[serde(deserialize_with = "crate::utils::empty_string_as_none")]
    pub sequence: Option<u16>,
    pub group: Option<String>,
    pub slug: String,
    pub song: Option<Song>,
    pub anime: Option<Anime>,
    #[serde(alias = "animethemeentries")]
    pub entries: Option<Vec<ThemeEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ThemeType {
    OP,
    ED,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Song {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub title: String,
    pub artists: Option<Vec<Artist>>,
    #[serde(alias = "animethemes")]
    pub themes: Option<Vec<Theme>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub name: String,
    pub slug: String,
    #[serde(alias = "as")]
    pub as_character: Option<String>,
    pub songs: Option<Vec<Song>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeEntry {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    #[serde(deserialize_with = "crate::utils::empty_string_as_none")]
    pub version: Option<u32>,
    pub episodes: String,
    pub nsfw: bool,
    pub spoiler: bool,
    pub notes: Option<String>,
    pub videos: Option<Vec<Video>>,
    #[serde(alias = "animetheme")]
    pub theme: Option<Theme>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub basename: String,
    pub filename: String,
    pub path: String,
    pub resolution: u32,
    #[serde(alias = "nc")]
    pub no_credits: bool,
    pub subbed: bool,
    pub lyrics: bool,
    #[serde(alias = "uncen")]
    pub uncensored: bool,
    #[serde(deserialize_with = "crate::utils::empty_string_as_none")]
    pub source: Option<VideoSource>,
    pub overlap: VideoOverlap,
    pub link: String,
    #[serde(alias = "animethemeentries")]
    pub entries: Option<Vec<ThemeEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VideoSource {
    WEB,
    RAW,
    BD,
    DVD,
    VHS,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VideoOverlap {
    None,
    Transition,
    Over,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Series {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub name: String,
    pub slug: String,
    pub anime: Option<Vec<Anime>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub link: String,
    pub external_id: u32,
    pub site: String,
    #[serde(alias = "as")]
    pub resource_as: Option<String>,
    pub anime: Option<Vec<Anime>>,
    pub artists: Option<Vec<Artist>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub meta: EntryMetadata,
    pub path: String,
    pub facet: ImageFacet,
    pub anime: Option<Vec<Anime>>,
    pub artists: Option<Vec<Artist>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ImageFacet {
    #[serde(alias = "Small Cover")]
    SmallCover,
    #[serde(alias = "Large Cover")]
    LargeCover,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub anime: Option<Vec<Anime>>,
    pub artists: Option<Vec<Artist>>,
    pub series: Option<Vec<Series>>,
    pub songs: Option<Vec<Song>>,
    #[serde(alias = "animethemes")]
    pub themes: Option<Vec<Theme>>,
    pub videos: Option<Vec<Video>>,
}
