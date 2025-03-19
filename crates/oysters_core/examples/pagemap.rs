extern crate oysters_core;
use oysters_core::*;

fn main() {
    let mut book: pagemap::PageMap<String, String> =
        pagemap::PageMap::new(pagemap::PageMapOptions {
            pages: 1,
            page_size: 8_000,
        });

    book.pagebook.insert(b"test", b"Hello, world!");
    let v = book.pagebook.get(b"test");
    dbg!(&String::from_utf8(v.unwrap()));

    book.pagebook.insert(b"test1", b"Hello, world! 1");
    let v = book.pagebook.get(b"test1");
    dbg!(&String::from_utf8(v.unwrap()));
}
