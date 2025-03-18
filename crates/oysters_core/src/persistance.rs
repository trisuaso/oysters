use crate::oyster::Oyster;
use pathbufd::PathBufD;
use rusqlite::{Connection, Result};
use std::hash::Hash;
use std::{fs, sync::LazyLock};

#[cfg(not(feature = "sqlite_backend"))]
use crate::pearl::{Pearl, ResourceDescriptor};

pub static PATH: LazyLock<PathBufD> = LazyLock::new(|| PathBufD::current().extend(&["dump.db"]));

impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    /// Dump the cache into the dump file ([`PATH`]).
    #[cfg(not(feature = "sqlite_backend"))]
    pub fn dump(&self) -> Result<()> {
        // create database file
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            if let Err(e) = fs::write(PATH.to_string(), []) {
                panic!("{}", e);
            };
        }

        // create database
        let conn = Connection::open(PATH.as_ref())?;
        conn.pragma_update(None, "journal_mode", "WAL")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        )?;

        for (k, v) in &self.0 {
            self.write_into_db(k, v).unwrap();
        }

        // return
        Ok(())
    }

    /// Read the dump file ([`PATH`]) and populate the map.
    #[cfg(not(feature = "sqlite_backend"))]
    pub fn restore(&mut self) -> Result<()> {
        // if the dump file doesn't even exist, just say we restored successfully
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            return Ok(());
        }

        // get database connection
        let conn = Connection::open(PATH.as_ref())?;

        // pull data
        let mut query = conn.prepare("SELECT \"key\", \"value\", \"used\" FROM \"map\"")?;
        let iter = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, usize>(2)?,
            ))
        })?;

        for i in iter {
            let (key, value, used) = i.unwrap();
            self.insert_full(
                key.into(),
                Pearl(
                    value.into(),
                    ResourceDescriptor {
                        #[cfg(feature = "lru")]
                        used,
                    },
                ),
            );
        }

        // return
        Ok(())
    }

    /// Write a key into the database.
    #[cfg(not(feature = "sqlite_backend"))]
    pub fn write_into_db(&self, k: &K, v: &Pearl<V>) -> Result<()> {
        let conn = Connection::open(PATH.as_ref())?;

        // check if key exists
        let mut query = conn.prepare("SELECT * FROM \"map\" WHERE \"key\" = ?")?;
        let selected = query.query_row([k.to_string()], |row| {
            Ok((
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, usize>(2)?,
            ))
        });

        if selected.is_err() {
            // doesn't exist yet
            conn.execute(
                "INSERT INTO \"map\" VALUES (?, ?, ?)",
                (k.to_string(), v.0.to_string(), v.1.used),
            )?;
        } else if let Ok(existing) = selected {
            // exists; update only if changed
            if (existing.1 != v.0.to_string()) | (existing.2 != v.1.used) {
                conn.execute(
                    "UPDATE \"map\" SET \"value\" = ?, \"used\" = ? WHERE \"key\" = ?",
                    (v.0.to_string(), v.1.used, k.to_string()),
                )?;
            }
        }

        Ok(())
    }

    /// Delete a key from the database.
    #[cfg(not(feature = "sqlite_backend"))]
    pub fn remove_from_db(&self, key: &K) -> Result<()> {
        // if the dump file doesn't even exist, just say we removed the key successfully
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            return Ok(());
        }

        // remove key
        let conn = Connection::open(PATH.as_ref())?;
        conn.execute("DELETE FROM \"map\" WHERE \"key\" = ?", [key.to_string()])?;

        // return
        Ok(())
    }

    /// Dump the cache into the dump file ([`PATH`]).
    #[cfg(feature = "sqlite_backend")]
    pub fn dump(&self) -> Result<()> {
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            if let Err(e) = fs::write(PATH.to_string(), []) {
                panic!("{}", e);
            };
        }

        let conn = Connection::open(PATH.as_ref())?;
        conn.pragma_update(None, "journal_mode", "WAL")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        )?;

        conn.execute("ATTACH DATABASE 'staging.db' AS staging_db", [])?;
        conn.execute("INSERT INTO map SELECT * FROM staging_db.map", [])?;

        Ok(())
    }

    /// Read the dump file ([`PATH`]) and populate the map.
    #[cfg(feature = "sqlite_backend")]
    pub fn restore(&mut self) -> Result<()> {
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            if let Err(e) = fs::write(PATH.to_string(), []) {
                panic!("{}", e);
            };

            let conn = Connection::open(PATH.as_ref())?;
            conn.pragma_update(None, "journal_mode", "WAL")?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS map (
                    key   TEXT NOT NULL,
                    value TEXT NOT NULL,
                    used  INT
                )",
                (),
            )?;
        }

        let conn = crate::sqlite_backend::connect()?;
        conn.execute("ATTACH DATABASE 'dump.db' AS disk_db", [])?;
        conn.execute("INSERT INTO map SELECT * FROM disk_db.map", [])?;

        Ok(())
    }
}
