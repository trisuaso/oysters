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
}
