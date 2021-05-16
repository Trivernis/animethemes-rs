use crate::client::AnimeThemesClient;

#[tokio::test]
async fn it_searches() {
    let client = AnimeThemesClient::default();
    let result = client.search("Vivy", &[], &[]).await.unwrap();
    assert!(result.entries.is_some());
    assert!(result.artists.is_some());
    assert!(result.songs.is_some());
    assert!(result.anime.is_some());
    assert!(result.series.is_some());
    assert!(result.synonyms.is_some());
    assert!(result.themes.is_some());
    assert!(result.videos.is_some());
}

#[tokio::test]
async fn it_returns_anime_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client
        .anime("vivy_fluorite_eyes_song", &["themes"])
        .await
        .unwrap();

    assert!(result.themes.is_some());
}

#[tokio::test]
async fn it_returns_artists_by_slug() {
    let client = AnimeThemesClient::default();
    let result = client.artist("lisa", &["songs"]).await.unwrap();

    assert!(result.songs.is_some());
}

#[tokio::test]
async fn it_returns_entries_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.entry(11948, &["videos"]).await.unwrap();

    assert!(result.videos.is_some())
}

#[tokio::test]
async fn it_returns_images_by_id() {
    let client = AnimeThemesClient::default();
    let result = client.image(7247, &["anime"]).await.unwrap();

    assert!(result.anime.is_some())
}
