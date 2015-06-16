extern crate json;

fn main() {

    let json = json::parser::parse("{\"key\": 42, \"key2\": {\"key3\": \"lol\"}, \"key4\": [\"lol\",1,2,3,45,\"lol\",\"pow!\",\"noice\"]}");
    println!("{:?}", json);
}
