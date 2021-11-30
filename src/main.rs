use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::<String, String>::new();

    map.insert("one".to_string(), "one".to_string());
    map.insert("one1".to_string(), "one1".to_string());
    map.insert("one2".to_string(), "one2".to_string());

    if let Some(s) = map.get_mut(&String::from("one")) {
        s.push_str("_追加字符串");
    };

    println!("map is : {:?}", map);
}
