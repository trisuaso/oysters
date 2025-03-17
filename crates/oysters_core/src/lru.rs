use crate::oyster::Oyster;
use crate::pearl::{EPOCH_YEAR, Pearl, ResourceDescriptor};
use crate::time::epoch_timestamp;
use std::collections::HashMap;
use std::hash::Hash;

impl<K, V> Oyster<K, V>
where
    K: Hash + Ord + Clone + Send + ToString + From<String>,
    V: Clone + Send + ToString + From<String>,
{
    /// Update the resource descriptor of an item. This method assumes that the item
    /// exists already, and thus will panic if it doesn't.
    ///
    /// # Arguments
    /// * `key` - the key the item is stored in
    pub fn update_resource_descriptor(&mut self, key: &K) {
        let item = self.0.get_mut(key).unwrap();
        item.1 = ResourceDescriptor::default();
    }

    /// [`Self::scan_sync`] backend.
    pub fn scan_with(map: &mut HashMap<K, Pearl<V>>) {
        let now = epoch_timestamp(EPOCH_YEAR);
        const MAXIMUM_AGE: usize = 604800000; // 7 days

        let clone = map.clone();
        for (k, item) in &clone {
            if (now - item.1.used) > MAXIMUM_AGE {
                map.remove(k);
            }
        }
        drop(clone);
    }

    /// Scan the entire map for outdated items (and remove them).
    pub fn scan_sync(&mut self) {
        let mut clone = self.0.clone();
        Oyster::scan_with(&mut clone);
        self.0 = clone;
    }

    /// [`Self::scan_sync`] but async.
    pub async fn scan(&mut self) {
        self.scan_sync();
    }
}
