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
        Null
    }

    pub type Array = Vec<Json>;
    pub type Object = BTreeMap<string::String, Json>;

    fn parse_value(it: &mut Chars) {
        match it.next() {
            Some(' ') => parse_value(it),
            Some('{') => parse_object(it),
            Some(c) => {
                if c.is_digit(10) {
                    let mut num_str = c.to_string();
                    for car in it.take_while(|a| a.is_digit(10)) {
                        num_str.push(car);
                    }
                    let num = num_str.parse::<u32>().unwrap();
                }
            },
            None => println!("Error occured")
        }
    }

    fn parse_object(it: &mut Chars) {
        let mut collector = "".to_string();
        loop {
            match it.next() {
                Some(':') => {
                    println!("Key is ready: {}", collector);
                    collector = "".to_string();
                    parse_value(it);
                },
                Some(',') => collector = "".to_string(),
                Some('}') => break,
                Some(c) => {
                    collector.push(c);
                    println!("Collecting: {}", collector);
                },
                None => break
            }
        }
    }

    pub fn parse(json: &str) {
        let finished = false;
        let mut it = json.chars();

        while !finished {
            match it.next() {
                Some('{') => parse_object(&mut it),
                _ => break
            };
        }
    }
}

#[test]
fn it_works() {
}
