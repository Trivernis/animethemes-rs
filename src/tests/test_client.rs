use crate::client::AnimeThemesClient;
use crate::includes::*;

#[tokio::test]
async fn it_searches() {
    let client = AnimeThemesClient::default();
    let result = client
        .search(
            "vivy",
            &[],
            SearchIncludes {
                anime: AnimeInclude::default().images().series(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
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
        .anime("vivy_fluorite_eyes_song", AnimeInclude::default().themes())
        .await
        .unwrap();

    assert!(result.themes.is_some());
}

#[tokio::test]
async fn it_returns_artists_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client
        .artist("lisa", ArtistInclude::default().songs())
        .await
        .unwrap();

    assert!(result.songs.is_some());
}

#[tokio::test]
async fn it_returns_entries_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .entry(11948, ThemeEntryInclude::default().theme().videos())
        .await
        .unwrap();

    assert!(result.videos.is_some());
    assert!(result.theme.is_some());
}

#[tokio::test]
async fn it_returns_images_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .image(7247, ImageInclude::default().anime())
        .await
        .unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_resources_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .resource(3588, ResourceInclude::default().anime())
        .await
        .unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_series_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client
        .series("shingeki_no_kyojin", SeriesInclude::default().anime())
        .await
        .unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_synonyms_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .synonym(2462, SynonymInclude::default().anime())
        .await
        .unwrap();

    assert!(result.anime.is_some())
}

#[tokio::test]
async fn it_returns_songs_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .song(8188, SongInclude::default().themes())
        .await
        .unwrap();

    assert!(result.themes.is_some())
}

#[tokio::test]
async fn it_returns_themes_by_id() {
    let client = AnimeThemesClient::default();
    let result = client
        .theme(8187, ThemeInclude::default().entries())
        .await
        .unwrap();

    assert!(result.entries.is_some())
}

#[tokio::test]
async fn it_returns_videos_by_basename() {
    let client = AnimeThemesClient::default();
    let result = client
        .video("KimiUso-OP2.webm", VideoInclude::default().entries())
        .await
        .unwrap();

    assert!(result.entries.is_some())
}
