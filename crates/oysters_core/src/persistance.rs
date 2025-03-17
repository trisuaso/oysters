use crate::oyster::Oyster;
use crate::pearl::{Pearl, ResourceDescriptor};
use pathbufd::PathBufD;
use rusqlite::{Connection, Result};
use std::hash::Hash;
use std::{fs, sync::LazyLock};

pub static PATH: LazyLock<PathBufD> = LazyLock::new(|| PathBufD::current().extend(&["dump.db"]));

impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    /// Dump the cache into the dump file ([`PATH`]).
    pub fn dump(&self) -> Result<()> {
        // create database file
        if !fs::exists(PATH.as_ref()).unwrap_or(false) {
            if let Err(e) = fs::write(PATH.to_string(), []) {
                panic!("{}", e);
            };
        }

        // create database
        let conn = Connection::open(PATH.as_ref())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        )?;

        for (k, v) in &self.0 {
            // check if key exists
            let mut query =
                conn.prepare("SELECT \"key\", \"value\" FROM \"map\" WHERE \"key\" = ? LIMIT 1")?;
            let selected = query.query([k.to_string()]).expect("failed to select");

            let mut vec = Vec::new();
            for row in selected.mapped(|row| {
                Ok((
                    row.get::<usize, String>(0)?,
                    row.get::<usize, String>(1)?,
                    row.get::<usize, usize>(2)?,
                ))
            }) {
                vec.push(row.unwrap());
            }

            if vec.len() == 0 {
                // doesn't exist yet
                conn.execute(
                    "INSERT INTO \"map\" VALUES (?, ?, ?)",
                    (k.to_string(), v.0.to_string(), v.1.used),
                )?;
            } else {
                // exists; update only if changed
                let existing = vec.get(0).unwrap();

                if (existing.1 != v.0.to_string()) | (existing.2 != v.1.used) {
                    conn.execute(
                        "UPDATE \"map\" SET \"value\" = ?, \"used\" = ? WHERE \"key\" = ?",
                        (v.0.to_string(), v.1.used, k.to_string()),
                    )?;
                }
            }
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
