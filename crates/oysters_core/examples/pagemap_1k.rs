extern crate oysters_core;
use oysters_core::*;

fn main() {
    let mut book: pagemap::PageMap<String, String> =
        pagemap::PageMap::new(pagemap::PageMapOptions {
            pages: 1,
            page_size: 16_000,
        });

    for i in 0..1 {
        let string = i.to_string();
        book.insert(string.to_string(), string);
    }
    book.insert("2".to_string(), "Hello, world!".to_string());

    // verify that we actually inserted everything by checking a random number
    // let v = book.get(&"514".to_string());
    // dbg!(&v);

    // book.remove(&"514".to_string());

    let v = book.get(&"514".to_string());
    dbg!(&v);

    book.dump(pathbufd::PathBufD::current().extend(&["page_dump"]));
}
