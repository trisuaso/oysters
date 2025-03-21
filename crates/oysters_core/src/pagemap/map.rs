use pathbufd::PathBufD;

use super::pagebook::PageBook;
use std::marker::PhantomData;

/// General options for a [`PageMap`].
pub struct PageMapOptions {
    /// The number of pages in the map.
    pub pages: usize,
    /// The size of each page in the map.
    pub page_size: usize,
}

/// A map which stores data as literal bytes split across multiple pages of the defined size.
pub struct PageMap<K, V>
where
    K: Ord + Clone + Send + ToString + From<String>,
    V: Ord + Clone + Send + ToString + From<String>,
{
    /// The pagebook stores pages in the map.
    pub pagebook: PageBook,
    /// The general options of the map.
    pub options: PageMapOptions,
    /// Phantom data. Should always be `None`.
    _phantoms: Option<(PhantomData<K>, PhantomData<V>)>,
}

impl<K, V> PageMap<K, V>
where
    K: Ord + Clone + Send + ToString + From<String>,
    V: Ord + Clone + Send + ToString + From<String>,
{
    /// Create a new [`PageMap`].
    pub fn new(options: PageMapOptions) -> Self {
        Self {
            pagebook: PageBook::new(options.pages, options.page_size),
            options,
            _phantoms: None,
        }
    }

    /// [`PageBook::get`]
    pub fn get(&self, key: &K) -> Option<V> {
        let x = self.pagebook.get(key.to_string().as_bytes());

        if let Some(v) = x {
            let string = String::from_utf8(v).unwrap();
            Some(string.into())
        } else {
            None
        }
    }

    /// [`PageBook::insert`]
    pub fn insert(&mut self, key: K, value: V) -> () {
        // check if the value already exists
        if self.get(&key).is_some() {
            return;
        }

        // insert
        self.pagebook
            .insert(key.to_string().as_bytes(), value.to_string().as_bytes())
    }

    /// [`PageBook::remove`]
    pub fn remove(&mut self, key: &K) -> Option<()> {
        self.pagebook.remove(key.to_string().as_bytes())
    }

    /// Dump the map into the given file at `path`.
    pub fn dump(&self, path: PathBufD) {
        for (i, page) in self.pagebook.0.iter().enumerate() {
            std::fs::write(path.join(format!("{i}.page")), page).unwrap()
        }
    }
}
