use rusqlite::Error;

use crate::{
    db::Database,
    error::{self},
    models::Track,
};

pub fn get_tracks(db: &Database) -> error::Result<Vec<Track>> {
    let mut stmt = db
        .conn
        .prepare("SELECT id, name, location, duration_seconds AS duration FROM tracks")?;

    let tracks = stmt
        .query_map([], |row| {
            Ok(Track {
                id: row.get("id")?,
                name: row.get("name")?,
                location: row.get("location")?,
                duration: row.get("duration")?,
            })
        })?
        .collect::<Result<Vec<Track>, Error>>()?;
    Ok(tracks)
}
