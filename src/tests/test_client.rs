use crate::client::AnimeThemesClient;
use crate::includes::{ANIME, SONGS, THEME, THEMES, THEME_ENTRIES, VIDEOS};

#[tokio::test]
async fn it_searches() {
    let client = AnimeThemesClient::default();
    let result = client.search("Vivy", &[], &[]).await.unwrap();
    assert!(result.artists.is_some());
    assert!(result.songs.is_some());
    assert!(result.anime.is_some());
    assert!(result.series.is_some());
    assert!(result.themes.is_some());
    assert!(result.videos.is_some());
}

#[tokio::test]
async fn it_returns_anime_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client
        .anime("vivy_fluorite_eyes_song", &[THEMES])
        .await
        .unwrap();

    assert!(result.themes.is_some());
}

#[tokio::test]
async fn it_returns_artists_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client.artist("lisa", &[SONGS]).await.unwrap();

    assert!(result.songs.is_some());
}

#[tokio::test]
async fn it_returns_entries_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.entry(11948, &[VIDEOS, THEME]).await.unwrap();

    assert!(result.videos.is_some());
    assert!(result.theme.is_some());
}

#[tokio::test]
async fn it_returns_images_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.image(7247, &[ANIME]).await.unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_resources_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.resource(3588, &[ANIME]).await.unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_series_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client.series("shingeki_no_kyojin", &[ANIME]).await.unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_synonyms_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.synonym(2462, &[ANIME]).await.unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_songs_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.song(8188, &[THEMES]).await.unwrap();

    assert!(result.themes.is_some())
}

#[tokio::test]
async fn it_returns_themes_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.theme(8187, &[THEME_ENTRIES]).await.unwrap();

    assert!(result.entries.is_some())
}

#[tokio::test]
async fn it_returns_videos_by_basename() {
    let client = AnimeThemesClient::default();
    let result = client
        .video("KimiUso-OP2.webm", &[THEME_ENTRIES])
        .await
        .unwrap();

    assert!(result.entries.is_some())
}
