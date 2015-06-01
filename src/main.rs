extern crate json;

fn main() {
    let json = json::parser::parse("{'key': 42}");
    println!("{:?}", json);
}
