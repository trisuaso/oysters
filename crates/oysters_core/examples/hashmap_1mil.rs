use std::collections::HashMap;

fn main() {
    let mut book: HashMap<String, String> = HashMap::new();

    for i in 0..1_000_000_i32 {
        book.insert(i.to_string(), i.to_string());
    }
}
