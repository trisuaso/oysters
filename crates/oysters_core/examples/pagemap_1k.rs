extern crate oysters_core;
use oysters_core::*;

fn main() {
    let mut book: pagemap::PageMap<String, String> =
        pagemap::PageMap::new(pagemap::PageMapOptions {
            pages: 1,
            page_size: 16_000,
        });

    for i in 0..1_000_i32 {
        let string = i.to_string();
        let bytes = string.as_bytes();
        book.pagebook.insert(bytes, bytes);
    }

    // verify that we actually inserted everything by checking a random number
    let v = book.pagebook.get(&[53, 49, 52]);
    dbg!(&String::from_utf8(v.unwrap()));
}
