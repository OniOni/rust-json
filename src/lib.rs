#![feature(slicing_syntax)]

extern crate core;

pub mod parser {

    use core::str::Chars as Chars;
    use std::collections::BTreeMap;
    use std::string;

    pub enum Json {
        String(string::String),
        Object(self::Object),
        Array(self::Array),
        U64(u64),
        Null
    }

    pub type Array = Vec<Json>;
    pub type Object = BTreeMap<string::String, Json>;

    fn parse_value(it: &mut Chars) -> Json {
        match it.next() {
            Some(' ') => parse_value(it),
            Some('{') => parse_object(it),
            Some(c) => {
                if c.is_digit(10) {
                    let mut num_str = c.to_string();
                    for car in it.take_while(|a| a.is_digit(10)) {
                        num_str.push(car);
                    }
                    Json::U64(num_str.parse::<u64>().unwrap())
                } else {
                    Json::Null
                }
            },
            None => Json::Null
        }
    }

    fn parse_object(it: &mut Chars) -> Json {
        let mut collector = "".to_string();
        let mut object = BTreeMap::new();
        loop {
            match it.next() {
                Some(':') => {
                    object.insert(collector, parse_value(it));
                    collector = "".to_string();
                },
                Some(',') => {collector = "".to_string()},
                Some('}') => break,
                Some(c) => {
                    collector.push(c);
                },
                None => break
            }
        }
        Json::Object(object)
    }

    pub fn parse(json: &str) -> Json {
        let mut it = json.chars();

        loop {
            match it.next() {
                Some('{') => return parse_object(&mut it),
                _ => return Json::Null
            }
        }
    }
}

#[test]
fn it_works() {
}
