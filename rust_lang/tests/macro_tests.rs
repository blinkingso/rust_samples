//! Rust Macros.
#![recursion_limit = "256"]
#[allow(unstable_features)]
pub mod macros;
macro_rules! http_status {
    ($status_code: expr, $message: expr) => {
        HttpStatus {
            status_code: $status_code,
            message: $message.to_string(),
        }
    };
}

struct HttpStatus {
    status_code: u16,
    message: String,
}

#[test]
fn test_http_status() {
    let hs = http_status!(100, "nothing to say");
    let hs1 = http_status!(200, String::from("OK"));

    // print the current filename;
    let f = file!();
    println!("{}", f);
    let line = line!();
    let column = column!();
    let s = stringify!(file!(), line!(), column!(), "this is for test", asd,);
    println!("{}", s);
    // concat literals only.
    let s = concat!("hello world", "hsdfa", 123, 1.23);
    let java_home = env!("PATH", "PATH NOT FOUND.");
    println!("java home : {}", java_home);
    let version = env!("CARGO_PKG_VERSION");
    println!("version : {}", version);
    // expanded at compile time.
    let rs: &'static str = include_str!("closures.rs");

    let s = "hello";
    if matches!(s, "hello") {
        println!("ok");
    }

    // let stx = log_syntax!("http_status");
}

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

use std::boxed::Box;
use std::collections::HashMap;
use std::string::ToString;
#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
    ([ $( $element:tt ),+ , ]) => {
        json!([ $( $element ),* ])
    };
    ({ $( $key:tt: $value:tt ),*}) => {
        // Json::Object(Box::new(vec![ $( ($key.to_string(), json!($value)) ),* ].into_iter().collect()))
        {
            let mut fields = $crate::macros::Box::new($crate::macros::HashMap::new());
            $( fields.insert($crate::macros::ToString::to_string($key), json!($value)); )*
            $crate::Json::Object(fields)
        }
    };
    ({ $( $key:tt: $value:tt ),+ , }) => {
        json!({ $( $key: $value ),* })
    };
    ( $other:tt ) => {
        // numbers, boolean. String....
        $crate::Json::from($other)
    };
}

macro_rules! impl_from_num_for_json {
    ( $( $t: ident )* ) => {
        $(
        impl From<$t> for Json {
            fn from(n: $t) -> Json {
                Json::Number(n as f64)
            }
        }
        )*
    }
}
impl_from_num_for_json!(u8 i8 i16 u16 i32 u32 i64 u64 u128 i128 usize isize f32 f64);
impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}
impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Boolean(b)
    }
}
impl From<String> for Json {
    fn from(string: String) -> Json {
        Json::String(string)
    }
}
#[test]
fn test_json() {
    let json = json!({
        "name": null,
        "width": 12,
        "height": 12.01,
        "rectangles": {
            "w": "120",
            "h": "220",
            "square": false,
            "name": null,
        },
        "pixels": [
            {"x": 1, "y": 2},
            {"x": 3, "y": 2},
        ]
    });

    println!("json: {:?}", json);

    let fields = "Fields, ....c";
    let role1 = json!({
        "name": "sdfa",
        "actor": fields
    });
    println!("{:?}", role1);
}
#[macro_use]
extern crate hello_macro;
use hello_macro::HelloMacro;
#[derive(HelloMacro)]
struct Pancakes;
#[derive(HelloMacro)]
struct HelloWorld {
    name: String,
}
#[test]
fn test_hello_macro() {
    let p = Pancakes;
    p.hello_macro();
    let h = HelloWorld {
        name: String::from("hello world"),
    };
    h.hello_macro();
}
