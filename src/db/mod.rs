use include_dir::{Dir, include_dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/migrations");

fn migrations() -> Migrations<'static> {
    Migrations::from_directory(&MIGRATIONS_DIR).unwrap()
}

pub struct Database {
    pub conn: Connection,
}
impl Database {
    pub fn open(path: &std::path::Path) -> crate::error::Result<Self> {
        let mut conn = Connection::open(path)?;

        conn.pragma_update(None, "foreign_keys", true)?;

        migrations().to_latest(&mut conn).unwrap();
        Ok(Self { conn })
    }
}
