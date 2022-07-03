macro_rules! theme_include {
    (
        pub struct $name: ident ($include_type:literal) {
            $($field: ident: $field_name: literal),+
        }
    ) => {
        #[derive(Clone, Copy, Default, Debug)]
        pub struct $name {
            $($field: bool),+
        }

        impl $name {
            $(pub fn $field(mut self) -> Self {
                self.$field = true;

                self
            }
            )+
        }

        impl Includes for $name {
            fn include_type() -> &'static str {
                $include_type
            }

            fn includes(&self) -> Vec<String> {
                let mut includes = Vec::new();
                $(
                    if self.$field {
                        includes.push($field_name.into());
                    }
                )+

                includes
            }
        }
    }
}

pub trait Includes {
    fn include_type() -> &'static str;
    fn includes(&self) -> Vec<String>;
}

theme_include!(
    pub struct AnimeInclude ("anime") {
        synonyms: "animesynonyms",
        themes: "animethemes",
        themes_entries: "animethemes.animethemeentries",
        themes_entries_videos: "animethemes.animethemeentries.videos",
        themes_song: "animethemes.song",
        themes_song_artists: "animethemes.song.artists",
        images: "images",
        resources: "resources",
        series: "series",
        studios: "studios"
    }
);

theme_include!(
    pub struct AnimeImageInclude ("animeimage") {
        anime: "anime",
        image: "image"
    }
);

theme_include!(
    pub struct SynonymInclude ("animesynonym") {
        anime: "anime"
    }
);

theme_include!(
    pub struct ThemeInclude ("animetheme") {
        anime: "anime",
        anime_images: "anime.images",
        entries: "animethemeentries",
        entries_videos: "animethemeentries.videos",
        song: "song",
        song_artists: "song.artists"
    }
);

theme_include!(
    pub struct ThemeEntryInclude ("animethemeentry") {
        theme: "animetheme",
        theme_anime: "animetheme.anime",
        videos: "videos"
    }
);

theme_include!(
    pub struct ArtistInclude ("artist") {
        groups: "groups",
        members: "members",
        resources: "resources",
        songs: "songs",
        songs_themes: "songs.animethemes",
        songs_themes_anime: "songs.animethemes.anime"
    }
);

theme_include!(
    pub struct ImageInclude ("image") {
        anime: "anime",
        artists: "artists",
        studios: "studios"
    }
);

theme_include!(
    pub struct ResourceInclude ("resource") {
        anime: "anime",
        artists: "artists",
        studios: "studios"
    }
);

theme_include!(
    pub struct SeriesInclude ("series") {
        anime: "anime"
    }
);

theme_include!(
    pub struct SongInclude ("song") {
        themes: "animethemes",
        themes_anime: "animethemes.anime",
        artists: "artists"
    }
);

theme_include!(
    pub struct Studio ("studio") {
        anime: "anime",
        images: "images",
        resources: "resources"
    }
);

theme_include!(
    pub struct VideoInclude ("video") {
        entries: "animethemeentries",
        entries_theme: "animethemeentries.animetheme",
        entries_theme_anime: "animethemeentries.animetheme.anime"
    }
);

#[derive(Clone, Copy, Default, Debug)]
pub struct SearchIncludes {
    pub anime: AnimeInclude,
    pub animethemes: ThemeInclude,
    pub artists: ArtistInclude,
    pub series: SeriesInclude,
    pub songs: SongInclude,
    pub videos: VideoInclude,
}

impl SearchIncludes {
    pub fn indo_includes(self) -> Vec<(String, String)> {
        let mut includes = Vec::new();
        let anime_includes = self.anime.includes();
        let animetheme_includes = self.animethemes.includes();
        let artist_includes = self.artists.includes();
        let series_includes = self.series.includes();
        let song_includes = self.songs.includes();
        let video_includes = self.videos.includes();

        if !anime_includes.is_empty() {
            includes.push((
                format!("includes[{}]", AnimeInclude::include_type()),
                anime_includes.join(","),
            ));
        }
        if !animetheme_includes.is_empty() {
            includes.push((
                format!("includes[{}]", ThemeInclude::include_type()),
                animetheme_includes.join(","),
            ));
        }
        if !artist_includes.is_empty() {
            includes.push((
                format!("includes[{}]", ArtistInclude::include_type()),
                artist_includes.join(","),
            ));
        }
        if !series_includes.is_empty() {
            includes.push((
                format!("includes[{}]", SeriesInclude::include_type()),
                series_includes.join(","),
            ));
        }
        if !song_includes.is_empty() {
            includes.push((
                format!("includes[{}]", SongInclude::include_type()),
                song_includes.join(","),
            ));
        }
        if !video_includes.is_empty() {
            includes.push((
                format!("includes[{}]", VideoInclude::include_type()),
                video_includes.join(","),
            ));
        }

        includes
    }
}
