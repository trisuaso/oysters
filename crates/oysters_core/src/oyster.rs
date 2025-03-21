use std::hash::Hash;

#[cfg(feature = "sqlite_backend")]
use std::marker::PhantomData;

#[cfg(not(feature = "sqlite_backend"))]
use crate::pearl::Pearl;
#[cfg(not(feature = "sqlite_backend"))]
use std::collections::{HashMap, hash_map::IntoIter};

#[derive(Debug)]
pub struct Oyster<K, V>(
    #[cfg(not(feature = "sqlite_backend"))] pub(crate) HashMap<K, Pearl<V>>,
    #[cfg(feature = "sqlite_backend")] pub(crate) PhantomData<K>,
    #[cfg(feature = "sqlite_backend")] pub(crate) PhantomData<V>,
)
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>;

impl<K, V> Default for Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "sqlite_backend"))]
impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    /// Create a new [`Oyster`].
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value
    pub fn insert(&mut self, key: K, value: V) -> Option<Pearl<V>> {
        let v = Pearl::new(value);
        self.0.insert(key, v)
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value (as [`Pearl<V>`])
    pub fn insert_full(&mut self, key: K, value: Pearl<V>) -> Option<Pearl<V>> {
        self.0.insert(key, value)
    }

    /// Increment the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn incr(&mut self, key: K) -> Option<Pearl<V>> {
        let value = self.get(&key)?.to_string().parse::<usize>().unwrap();
        self.insert(key, (value + 1).to_string().into())
    }

    /// Decrement the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn decr(&mut self, key: K) -> Option<Pearl<V>> {
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
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(v) = self.0.get(key) {
            Some(&v.0)
        } else {
            None
        }
    }

    /// Get a full [`Pearl`] given its `key`.
    ///
    /// # Arguments
    /// * `key` - the key the value is stored in
    pub fn get_full(&self, key: &K) -> Option<&Pearl<V>> {
        self.0.get(key)
    }

    /// Get all items where their key starts with the given `prefix`.
    ///
    /// # Arguments
    /// * `prefix` - the prefix to match keys against
    pub fn starting_with(&self, prefix: &str) -> Vec<(&K, &Pearl<V>)> {
        let matches = self
            .0
            .iter()
            .filter(|x| x.0.to_string().starts_with(prefix));

        matches.collect()
    }

    /// Get all items where their key ends with the given `prefix`.
    ///
    /// # Arguments
    /// * `suffix` - the suffix to match keys against
    pub fn ending_with(&self, suffix: &str) -> Vec<(&K, &Pearl<V>)> {
        let matches = self.0.iter().filter(|x| x.0.to_string().ends_with(suffix));
        matches.collect()
    }

    /// Selects either [`Self::starting_with`] or [`Self::ending_with`], depending on if the given
    /// pattern string ends with `*`, or begins with `*` (respectively).
    ///
    /// # Arguments
    /// * `pattern` - the pattern to match keys against
    pub fn filter(&self, pattern: &str) -> Vec<(&K, &Pearl<V>)> {
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
    pub fn starting_with_keys(&self, prefix: &str) -> Vec<&K> {
        let matches = self.0.keys().filter(|x| x.to_string().starts_with(prefix));
        matches.collect()
    }

    /// Get all keys which end with the given `prefix`.
    ///
    /// # Arguments
    /// * `suffix` - the suffix to match keys against
    pub fn ending_with_keys(&self, suffix: &str) -> Vec<&K> {
        let matches = self.0.keys().filter(|x| x.to_string().ends_with(suffix));
        matches.collect()
    }

    /// Selects either [`Self::starting_with_keys`] or [`Self::ending_with_keys`], depending on if the given
    /// pattern string ends with `*`, or begins with `*` (respectively).
    ///
    /// # Arguments
    /// * `pattern` - the suffix to match keys against
    pub fn filter_keys(&self, pattern: &str) -> Vec<&K> {
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
    pub fn remove(&mut self, key: &K) -> Option<Pearl<V>> {
        #[cfg(feature = "persistance")]
        self.remove_from_db(key).unwrap();

        self.0.remove(key)
    }
}

#[cfg(not(feature = "sqlite_backend"))]
impl<K, V> IntoIterator for Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    type Item = (K, Pearl<V>);
    type IntoIter = IntoIter<K, Pearl<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
