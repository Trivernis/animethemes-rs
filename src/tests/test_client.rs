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
