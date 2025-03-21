extern crate oysters_core;
use oysters_core::*;

fn main() {
    let mut book: pagemap::PageMap<String, String> =
        pagemap::PageMap::new(pagemap::PageMapOptions {
            pages: 1,
            page_size: 64,
        });

    book.pagebook.insert(b"test", b"Hello, world!");
    let v = book.pagebook.get(b"test");
    dbg!(&String::from_utf8(v.unwrap()));

    book.pagebook.insert(b"test1", b"Hello, world! 1");
    let v = book.pagebook.get(b"test1");
    dbg!(&String::from_utf8(v.unwrap()));

    book.pagebook.remove(b"test");
    let v = book.pagebook.get(b"test");
    dbg!(v.is_some());

    book.pagebook.insert(b"test2", b"Hello, world! 2");
    let v = book.pagebook.get_full(b"test2").unwrap();
    dbg!(v.0, &String::from_utf8(v.1));

    book.dump(pathbufd::PathBufD::current().extend(&["page_dump"]));
}
