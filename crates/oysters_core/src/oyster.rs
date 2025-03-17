use crate::pearl::Pearl;
use std::{
    collections::{HashMap, hash_map::IntoIter},
    hash::Hash,
};

#[derive(Debug)]
pub struct Oyster<K, V>(pub(crate) HashMap<K, Pearl<V>>)
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

impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    pub type Item = Pearl<V>;

    /// Create a new [`Oyster`].
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value
    pub fn insert(&mut self, key: K, value: V) -> Option<Self::Item> {
        self.0.insert(key, Pearl::new(value))
    }

    /// Insert a value given its `key` and `value`.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    /// * `value` - the actual value (as [`Self::Item`])
    pub fn insert_full(&mut self, key: K, value: Self::Item) -> Option<Self::Item> {
        self.0.insert(key, value)
    }

    /// Increment the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn incr(&mut self, key: K) -> Option<Self::Item> {
        let value = self.get(&key)?
        .to_string()
        .parse::<usize>()
        .unwrap();

        self.insert(key, (value + 1).to_string().into())
    }

    /// Decrement the value of a key.
    ///
    /// # Arguments
    /// * `key` - the key to store the value in
    pub fn decr(&mut self, key: K) -> Option<Self::Item> {
        let mut value = self.get(&key)?
        .to_string()
        .parse::<usize>()
        .unwrap();

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
    pub fn get_full(&self, key: &K) -> Option<&Self::Item> {
        self.0.get(key)
    }

    /// Get all items where their key starts with the given `prefix`.
    ///
    /// # Arguments
    /// * `prefix` - the prefix to match keys against
    pub fn starting_with(&self, prefix: &str) -> Vec<(&K, &Self::Item)> {
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
    pub fn ending_with(&self, suffix: &str) -> Vec<(&K, &Self::Item)> {
        let matches = self.0.iter().filter(|x| x.0.to_string().ends_with(suffix));
        matches.collect()
    }

    /// Selects either [`Self::starting_with`] or [`Self::ending_with`], depending on if the given
    /// pattern string ends with `*`, or begins with `*` (respectively).
    ///
    /// # Arguments
    /// * `pattern` - the pattern to match keys against
    pub fn filter(&self, pattern: &str) -> Vec<(&K, &Self::Item)> {
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
    pub fn remove(&mut self, key: &K) -> Option<Self::Item> {
        self.0.remove(key)
    }
}

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
