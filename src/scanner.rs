use std::path::Path;

use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use walkdir::WalkDir;

use crate::{error, models::Track};

const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "ogg", "wav", "m4a", "opus", "aac"];

///Scans for audio files within a directory
///Only decends one directory to not go too deep
pub fn scan(dir: &Path) -> error::Result<Vec<Track>> {
    let mut tracks = Vec::<Track>::new();

    for entry in WalkDir::new(dir)
        .max_depth(2) //only go down one directory
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_lowercase);

        if !ext
            .as_deref()
            .is_some_and(|e| AUDIO_EXTENSIONS.contains(&e))
        {
            continue;
        }
        let Ok(tagged) = Probe::open(path).and_then(|p| p.read()) else {
            continue;
        };
        let mut name = tagged
            .primary_tag()
            .and_then(|t| t.title().map(|s| s.into_owned()))
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_owned()
            });
        name.truncate(20);
        let bytes = std::fs::metadata(path)?.len();

        let duration = tagged.properties().duration().as_secs_f32();
        tracks.push(Track {
            name,
            duration,
            location: path.to_string_lossy().into_owned(),
            bytes,
        });
    }

    Ok(tracks)
}
