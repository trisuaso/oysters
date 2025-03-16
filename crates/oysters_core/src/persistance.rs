use crate::oyster::Oyster;
use crate::pearl::{Pearl, ResourceDescriptor};
use pathbufd::PathBufD;
use rusqlite::{Connection, Result};
use std::{fs, sync::LazyLock};

pub static PATH: LazyLock<PathBufD> = LazyLock::new(|| PathBufD::current().extend(&["dump.db"]));

impl<K, V> Oyster<K, V>
where
    K: Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    /// Dump the cache into the dump file ([`PATH`]).
    pub fn dump(&self) -> Result<()> {
        // replace database file
        if fs::exists(PATH.as_ref()).unwrap_or(false) {
            if let Err(e) = fs::remove_file(PATH.as_ref()) {
                panic!("{}", e);
            };
        }

        if let Err(e) = fs::write(PATH.to_string(), []) {
            panic!("{}", e);
        };

        // create database
        let conn = Connection::open(PATH.as_ref())?;

        conn.execute(
            "CREATE TABLE map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        )?;

        for (k, v) in &self.0 {
            conn.execute(
                "INSERT INTO \"map\" VALUES (?, ?, ?)",
                (k.to_string(), v.0.to_string(), v.1.used),
            )?;
        }

        // return
        Ok(())
    }

    /// Read the dump file ([`PATH`]) and populate the map.
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
}
