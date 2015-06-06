extern crate json;

fn main() {
    for _ in 0..1000 {
        json::parser::parse("{\"key\": 42, \"key2\": {\"key3\": 1}}");
    }

    let json = json::parser::parse("{\"key\": 42, \"key2\": {\"key3\": \"lol\"}}");
    println!("{:?}", json);
}
