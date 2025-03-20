use lz4_flex::{compress_prepend_size as compress, decompress_size_prepended as decompress};
use std::ops::Range;
use std::ptr::{read as ptrread, write as ptrwrite};

pub struct PageBook(pub Vec<Vec<u8>>, pub (usize, usize, usize), (usize, usize));

impl PageBook {
    /// Create a new [`PageBook`].
    ///
    /// The given `count` is just the _initial_ number of pages in the pagebook.
    /// The book will grow if it runs out of room.
    pub fn new(count: usize, size: usize) -> Self {
        let mut pagebook = Vec::with_capacity(count);

        for _ in 0..count {
            pagebook.push(vec![0; size]);
        }

        Self(pagebook, (count, size, 0), (0, 0))
    }

    /// Create a new page under self.
    pub fn create_page(&mut self) -> Option<&Vec<u8>> {
        self.0.insert(0, vec![0; self.1.1]); // insert page TO THE BEGINNING (so it is filled ASAP)
        self.1.0 += 1; // incr page count
        self.0.get(self.1.0 - 1)
    }

    /// Check if the window contains all null bytes.
    pub(crate) fn check_window_for_not_null(window: &[u8]) -> bool {
        let sub_1 = window.len() - 1;
        let null = &0_u8;

        let first_byte = window.get(0).unwrap() == null;
        let last_byte = window.get(sub_1).unwrap() == null;
        let middle_byte = window.get(sub_1 / 2).unwrap() == null;

        first_byte && middle_byte && last_byte
    }

    /// Get the next free (\0) range of the given size in the given page.
    pub fn find_free_range(page: &[u8], size: usize) -> Option<Range<usize>> {
        let mut windows = page.windows(size);

        if let Some(i) = windows.position(|x| PageBook::check_window_for_not_null(x)) {
            return Some(i..i + size);
        }

        None
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
        let key = compress(key);
        for (i, page) in self.0.iter().enumerate() {
            let mut windows = page.windows(key.len()); // + 1 to include sep byte
            let mut window_num: usize = 0;
            while let Some(chunk) = windows.next() {
                let mut chunk: Vec<u8> = chunk.to_vec();

                if chunk.get(0).unwrap() == &2_u8 {
                    // chunks _might_ start with \2 because of being RIGHT after
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

    /// Get a value from the pagebook (assuming we **don't** know the page number).
    ///
    /// Returns the page number as part of the output.
    ///
    /// # Arguments
    /// * `key` - the key as bytes
    pub fn get_full(&self, key: &[u8]) -> Option<(usize, Vec<u8>)> {
        let page = match self.find_page(&key) {
            Some((page, _)) => page,
            None => return None,
        };

        if let Some(v) = self.get_known(page, key) {
            Some((page, v))
        } else {
            None
        }
    }

    /// Get the range of a key in the pagebook. Range includes from the start of
    /// the key, to the end of the value (with terminating byte).
    ///
    /// # Arguments
    /// * `page` - the page the key exists in
    /// * `key` - the key as bytes
    pub fn get_key_range(&self, page: usize, key: &[u8]) -> Option<Range<usize>> {
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

        let key_end = pos + key.len() + 1;

        // collect bytes to terminating byte
        let mut count: usize = 0;
        let sub: &[u8] = &page[key_end..page.len()];

        for byte in sub {
            count += 1;

            if byte == &2_u8 {
                break;
            }
        }

        // return
        Some(pos..count)
    }

    /// Get a value from the pagebook (assuming we know the page number).
    ///
    /// # Arguments
    /// * `page` - the page the key is contained in
    /// * `key` - the key as bytes
    pub fn get_known(&self, page: usize, key: &[u8]) -> Option<Vec<u8>> {
        let key = &compress(key);

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
        // (\2 terminates values, \1 terminates keys)
        let mut out: Vec<u8> = Vec::new();
        let sub: &[u8] = &page[key_end..page.len()];

        for byte in sub {
            if byte == &2_u8 {
                break;
            }

            out.push(byte.to_owned());
        }

        Some(decompress(&out).unwrap())
    }

    /// Insert a value into the book at the **first available** location.
    ///
    /// # Arguments
    /// * `key` - the key as bytes
    /// * `value` - the value as bytes
    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        let key = &compress(key);
        let value = &compress(value);

        // find good page
        let page: (usize, Option<Range<usize>>) = {
            let mut num: usize = 0;
            let mut range: Option<Range<usize>> = None;

            for (i, page) in self.0.iter().skip(self.2.0).enumerate() {
                let free_range = PageBook::find_free_range(page, key.len() + value.len() + 2);

                if free_range.is_none() {
                    // page is full
                    continue;
                }

                num = i;
                range = free_range;
            }

            self.2.0 = num; // store the last page we had to check as the skip value

            if range.is_none() {
                if self.create_page().is_some() {
                    // immediately set range to 0..(minimum needed)
                    // we're skipping `find_free_range` here because that will actually
                    // spend resources looking for the value, meanwhile we know the page
                    // is empty
                    range = Some(0..(key.len() + value.len() + 2));
                }
            }

            (num, range)
        };

        // select page as mutable
        let free_range = page.1.unwrap();
        let page = self.0.get_mut(page.0).unwrap();

        // push data
        let out: Vec<u8> = [key.as_slice(), &[1_u8], value.as_slice(), &[2_u8]].concat();

        // swap data
        let mut idx: usize = 0;
        for i in free_range {
            unsafe {
                let mut b = *out.get_unchecked(idx);
                ptrwrite(&mut page[i], ptrread(&mut b));
            }

            idx += 1;
        }

        // incr key count
        self.1.2 += 1;
    }

    /// Remove a value from the pagebook (assuming we **don't** know the page number).
    ///
    /// # Arguments
    /// * `key` - the key as bytes
    pub fn remove(&mut self, key: &[u8]) -> Option<()> {
        self.remove_known(
            match self.find_page(&key) {
                Some((page, _)) => page,
                None => return None,
            },
            key,
        )
    }

    /// Remove a key (and its value) from the given page.
    ///
    /// # Arguments
    /// * `page` - the page the key exists in
    /// * `key` - the key as bytes
    pub fn remove_known(&mut self, page: usize, key: &[u8]) -> Option<()> {
        // get range
        let range = match self.get_key_range(page, key) {
            Some(r) => r,
            None => return None,
        };

        // get mut page
        let page = match self.0.get_mut(page) {
            Some(p) => p,
            None => return None,
        };

        // remove
        let mut removed_bytes: usize = 0;
        for i in range {
            page.remove(i - removed_bytes); // subtract the number of bytes we've already removed to account for changing len
            removed_bytes += 1;
        }

        // decr key count
        self.1.2 -= 1;

        // return
        Some(())
    }
}
