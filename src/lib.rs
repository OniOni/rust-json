#![feature(collections)]

extern crate collections;

pub mod parser {

    use collections::fmt::{Formatter, Result};
    use std::fmt::Debug;
    use collections::str::Chars;
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

    impl Debug for Json {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match *self {
                Json::String(ref string) => String::fmt(string, f),
                Json::Object(ref obj) => BTreeMap::fmt(obj, f),
                Json::U64(ref int) => Debug::fmt(int, f),
                Json::Array(ref ar) => Debug::fmt(ar, f),
                _ => panic!("Noooo!")
            }
        }
    }

    fn parse_string(it: &mut Chars) -> String {
        let mut string = "".to_string();

        loop {
            match it.next() {
                Some('"') => {
                    break;
                },
                Some('\\') => continue,
                Some(c) => string.push(c),
                None => panic!("Reached end of iterator.")
            }
        }

        return string;
    }

    fn parse_int(it: &Chars, c: char) -> Json {
        let mut num_str = c.to_string();
        let clo = it.clone();
        for car in clo.take_while(|a| a.is_digit(10)) {
            num_str.push(car);
        }
        Json::U64(num_str.parse::<u64>().unwrap())
    }

    fn parse_value(it: &mut Chars) -> Json {
        match it.next() {
            Some(' ') => parse_value(it),
            Some('{') => parse_object(it),
            Some('"') => Json::String(parse_string(it)),
            Some('[') => parse_array(it),
            Some(c) => {
                if c.is_digit(10) {
                    parse_int(it, c)
                } else {
                    Json::Null
                }
            },
            None => Json::Null
        }
    }

    fn parse_array(it: &mut Chars) -> Json {
        let mut ar: Array = vec![parse_value(it)];

        loop {
            match it.next() {
                Some(']') => break,
                None => break,
                Some(',') => ar.push(parse_value(it)),
                Some(_) => continue
            }
        }
        Json::Array(ar)
    }

    fn parse_object(it: &mut Chars) -> Json {
        let mut object = BTreeMap::new();
        let mut key = "".to_string();

        loop {
            match it.next() {
                Some(':') => {
                    object.insert(key, parse_value(it));
                    key = "".to_string();
                },
                Some('"') => {
                    key = parse_string(it);
                },
                Some('}') => break,
                Some(_) => continue,
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
