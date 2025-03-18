use crate::oyster::Oyster;
use crate::{Pearl, pearl::ResourceDescriptor};
use rusqlite::{Connection, Result};
use std::hash::Hash;
use std::marker::PhantomData;

/// Obtain a connection to the staging database.
pub(crate) fn connect() -> Result<Connection> {
    Ok(Connection::open("staging.db")?)
}

impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    pub fn new() -> Self {
        let conn = connect().unwrap();
        conn.pragma_update(None, "journal_mode", "WAL").unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        )
        .unwrap();

        Self(PhantomData, PhantomData)
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value
    pub fn insert(&self, key: K, value: V) -> Option<()> {
        self.insert_full(key, Pearl::new(value))
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value (as [`Pearl<V>`])
    pub fn insert_full(&self, key: K, value: Pearl<V>) -> Option<()> {
        let conn = match connect() {
            Ok(c) => c,
            Err(_) => return None,
        };

        if let Err(_) = conn.execute(
            "CREATE TABLE IF NOT EXISTS map (
                key   TEXT NOT NULL,
                value TEXT NOT NULL,
                used  INT
            )",
            (),
        ) {
            return None;
        };

        // check if key exists
        let mut query = match conn.prepare("SELECT * FROM \"map\" WHERE \"key\" = ?") {
            Ok(q) => q,
            Err(_) => return None,
        };

        let selected = query.query_row([key.to_string()], |row| {
            Ok((
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, usize>(2)?,
            ))
        });

        if selected.is_err() {
            // doesn't exist yet
            if let Err(_) = conn.execute(
                "INSERT INTO \"map\" VALUES (?, ?, ?)",
                (key.to_string(), value.0.to_string(), value.1.used),
            ) {
                return None;
            };
        } else if let Ok(existing) = selected {
            // exists; update only if changed
            if (existing.1 != value.0.to_string()) | (existing.2 != value.1.used) {
                if let Err(_) = conn.execute(
                    "UPDATE \"map\" SET \"value\" = ?, \"used\" = ? WHERE \"key\" = ?",
                    (value.0.to_string(), value.1.used, key.to_string()),
                ) {
                    return None;
                };
            }
        }

        Some(())
    }

    /// Increment the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn incr(&mut self, key: K) -> Option<()> {
        let value = self.get(&key)?.to_string().parse::<usize>().unwrap();
        self.insert(key, (value + 1).to_string().into())
    }

    /// Decrement the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn decr(&mut self, key: K) -> Option<()> {
        let mut value = self.get(&key)?.to_string().parse::<usize>().unwrap();

        if value == 0 {
            value += 1; // this will make the value just end up as 0
        }

        self.insert(key, (value - 1).to_string().into())
    }

    /// Get a value given its `key`.
    ///
    /// # Arguments
    /// * `key` - the key the value is stored in
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(v) = self.get_full(key) {
            Some(v.0)
        } else {
            None
        }
    }

    /// Get a full [`Pearl`] given its `key`.
    ///
    /// # Arguments
    /// * `key` - the key the value is stored in
    pub fn get_full(&self, key: &K) -> Option<Pearl<V>> {
        let conn = match connect() {
            Ok(c) => c,
            Err(_) => return None,
        };

        let mut query = match conn.prepare("SELECT * FROM \"map\" WHERE \"key\" = ? LIMIT 1") {
            Ok(q) => q,
            Err(_) => return None,
        };

        match query.query_row([key.to_string()], |row| {
            Ok((
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, usize>(2)?,
            ))
        }) {
            Ok(r) => Some(Pearl(r.1.into(), ResourceDescriptor { used: r.2 })),
            Err(_) => None,
        }
    }

    /// Get all items where their key starts with the given `prefix`.
    ///
    /// # Arguments
    /// * `prefix` - the prefix to match keys against
    pub fn starting_with(&self, prefix: &str) -> Vec<(K, Pearl<V>)> {
        let conn = match connect() {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let mut query = match conn.prepare("SELECT * FROM \"map\" WHERE \"key\" LIKE ? LIMIT 1") {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        };

        let selected = query
            .query_map([format!("{prefix}%")], |row| {
                Ok((
                    row.get::<usize, String>(0)?,
                    row.get::<usize, String>(1)?,
                    row.get::<usize, usize>(2)?,
                ))
            })
            .expect("failed to query rows");

        let mut out = Vec::new();
        for row in selected {
            if let Ok(row) = row {
                out.push((
                    row.0.into(),
                    Pearl(row.1.into(), ResourceDescriptor { used: row.2 }),
                ))
            }
        }

        out
    }

    /// Get all items where their key ends with the given `prefix`.
    ///
    /// # Arguments
    /// * `suffix` - the suffix to match keys against
    pub fn ending_with(&self, suffix: &str) -> Vec<(K, Pearl<V>)> {
        let conn = match connect() {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let mut query = match conn.prepare("SELECT * FROM \"map\" WHERE \"key\" LIKE ? LIMIT 1") {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        };

        let selected = query
            .query_map([format!("%{suffix}")], |row| {
                Ok((
                    row.get::<usize, String>(0)?,
                    row.get::<usize, String>(1)?,
                    row.get::<usize, usize>(2)?,
                ))
            })
            .expect("failed to query rows");

        let mut out = Vec::new();
        for row in selected {
            if let Ok(row) = row {
                out.push((
                    row.0.into(),
                    Pearl(row.1.into(), ResourceDescriptor { used: row.2 }),
                ))
            }
        }

        out
    }

    /// Selects either [`Self::starting_with`] or [`Self::ending_with`], depending on if the given
    /// pattern string ends with `*`, or begins with `*` (respectively).
    ///
    /// # Arguments
    /// * `pattern` - the pattern to match keys against
    pub fn filter(&self, pattern: &str) -> Vec<(K, Pearl<V>)> {
        let pat = &pattern.replace("*", "");
        if pattern.starts_with("*") {
            self.ending_with(pat)
        } else {
            self.starting_with(pat)
        }
    }

    /// Get all keys which start with the given `prefix`.
    ///
    /// # Arguments
    /// * `prefix` - the prefix to match keys against
    pub fn starting_with_keys(&self, prefix: &str) -> Vec<K> {
        self.starting_with(prefix)
            .into_iter()
            .map(|x| x.0)
            .collect()
    }

    /// Get all keys which end with the given `prefix`.
    ///
    /// # Arguments
    /// * `suffix` - the suffix to match keys against
    pub fn ending_with_keys(&self, suffix: &str) -> Vec<K> {
        self.ending_with(suffix).into_iter().map(|x| x.0).collect()
    }

    /// Selects either [`Self::starting_with_keys`] or [`Self::ending_with_keys`], depending on if the given
    /// pattern string ends with `*`, or begins with `*` (respectively).
    ///
    /// # Arguments
    /// * `pattern` - the suffix to match keys against
    pub fn filter_keys(&self, pattern: &str) -> Vec<K> {
        let pat = &pattern.replace("*", "");
        if pattern.starts_with("*") {
            self.ending_with_keys(pat)
        } else {
            self.starting_with_keys(pat)
        }
    }

    /// Remove a value given its `key`.
    ///
    /// # Arguments
    /// * `key` - the key the value is stored in
    pub fn remove(&self, key: &K) -> Option<()>
    where
        K: Clone + ToString,
    {
        let conn = match connect() {
            Ok(c) => c,
            Err(_) => return None,
        };

        let mut query = match conn.prepare("DELETE FROM \"map\" WHERE \"key\" = ?") {
            Ok(q) => q,
            Err(_) => return None,
        };

        match query.execute([key.to_string()]) {
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }
}
