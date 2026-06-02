//! Database access and schema migration.

use include_dir::{Dir, include_dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/migrations");

/// Loads migrations from the embedded `src/migrations` directory.
fn migrations() -> Migrations<'static> {
    Migrations::from_directory(&MIGRATIONS_DIR).unwrap()
}

/// A wrapper around a SQLite [`Connection`] with the app schema applied.
pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Opens (or creates) the database at `path`, enables foreign-key enforcement,
    /// and runs any pending migrations.
    pub fn open(path: &std::path::Path) -> crate::error::Result<Self> {
        let mut conn = Connection::open(path)?;

        conn.pragma_update(None, "foreign_keys", true)?;

        migrations().to_latest(&mut conn).unwrap();
        Ok(Self { conn })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //validates the migrations are valid
    #[test]
    fn migrations_test() {
        assert!(migrations().validate().is_ok());
    }

    // Opens a real in-memory database end-to-end
    #[test]
    fn open_succeeds() {
        assert!(Database::open(std::path::Path::new(":memory:")).is_ok());
    }

    // Confirms the pragma_update call actually took effect
    #[test]
    fn foreign_keys_enabled() {
        let db = Database::open(std::path::Path::new(":memory:")).unwrap();
        let enabled: i32 = db
            .conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(enabled, 1);
    }
    // Exercises the down migration path
    #[test]
    fn migrations_up_and_down() {
        let mut conn = Connection::open(":memory:").unwrap();
        let m = migrations();
        m.to_latest(&mut conn).unwrap();
        assert!(m.to_version(&mut conn, 0).is_ok());
    }
}
