pub struct PageBook(Vec<Vec<u8>>, pub (usize, usize));

impl PageBook {
    /// Create a new [`PageBook`].
    pub fn new(count: usize, size: usize) -> Self {
        let mut pagebook = Vec::with_capacity(count);

        for _ in 0..count {
            pagebook.push(Vec::with_capacity(size));
        }

        Self(pagebook, (count, size))
    }

    /// Find the page the given `key` blongs to.
    ///
    /// Note that this will always return the **first** match. The same key _could_
    /// exist in other pages!
    ///
    /// # Returns
    /// Will return on option containing a tuple of the page number, followed by
    /// the window number.
    ///
    /// If the key doesn't exist in any page, `None` will be returned.
    pub fn find_page(&self, key: &[u8]) -> Option<(usize, usize)> {
        for (i, page) in self.0.iter().enumerate() {
            let mut windows = page.windows(key.len()); // + 1 to include sep byte
            let mut window_num: usize = 0;
            while let Some(chunk) = windows.next() {
                let mut chunk: Vec<u8> = chunk.to_vec();

                if chunk.get(0).unwrap() == &0_u8 {
                    // chunks _might_ start with \0 because of being RIGHT after
                    // a value chunk... we should just go ahead and remove it and ignore
                    chunk.remove(0);
                }

                if chunk == key {
                    return Some((i, window_num));
                }

                window_num += 1;
            }
        }

        None
    }

    /// Get a value from the pagebook (assuming we **don't** know the page number).
    ///
    /// # Arguments
    /// * `key` - the key as bytes
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get_known(
            match self.find_page(&key) {
                Some((page, _)) => page,
                None => return None,
            },
            key,
        )
    }

    /// Get a value from the pagebook (assuming we know the page number).
    ///
    /// # Arguments
    /// * `page` - the page the key is contained in
    /// * `key` - the key as bytes
    pub fn get_known(&self, page: usize, key: &[u8]) -> Option<Vec<u8>> {
        // get page
        let page = match self.0.get(page) {
            Some(p) => p,
            None => return None,
        };

        // get value
        let mut windows = page.windows(key.len());
        let pos = match windows.position(|x| x == key) {
            Some(p) => p,
            None => return None,
        };

        // now that we know the position, the actual STARTING INDEX of the window
        // is `pos + key.len()` ... this means that the ending pos is `pos + key.len() + 1`
        // (+ 1 for the \1 byte after the key)
        let key_end = pos + key.len() + 1;

        // we can get the actual data by getting every byte UNTIL a null byte is found
        // (\0 terminates values, \1 terminates keys)
        let mut out: Vec<u8> = Vec::new();
        let sub: &[u8] = &page[key_end..page.len()];

        for byte in sub {
            if byte == &0_u8 {
                break;
            }

            out.push(byte.to_owned());
        }

        Some(out)
    }

    /// Insert a value into the book at the **first available** location.
    ///
    /// # Arguments
    /// * `key` - the key as bytes
    /// * `value` - the value as bytes
    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        // find good page
        let page: usize = {
            let mut num: usize = 0;
            for (i, page) in self.0.iter().enumerate() {
                if page.len() == self.1.1 {
                    // page is full
                    continue;
                }

                num = i;
            }

            num
        };

        // select page as mutable
        let page = self.0.get_mut(page).unwrap();

        // push data
        for byte in key {
            page.push(byte.to_owned());
        }

        page.push(1_u8);

        for byte in value {
            page.push(byte.to_owned());
        }

        page.push(0_u8);
    }
}
