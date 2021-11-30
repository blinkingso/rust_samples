/// common collections: Vec<T> HashMap Strings.
fn main() {}

#[cfg(test)]
mod test {

    extern crate ansi_term;
    use self::ansi_term::{Colour, Style};
    use ansi_term::Colour::Red;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    #[test]
    fn test_vector_api() {
        // create a new vector
        let v = Vec::<i32>::new();
        // Rust can infer the type of value stored once you insert values.
        let v = vec![1, 2, 3, 4, 5];
        // update a vector;
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        // dropping a vector drops Its Elements;
        {
            let v = vec![1, 2, 3, 4, 5];
            // do stuff with v;
            // when the vector gets dropped, all of its contents are also dropped.
        } // <- v goes out of scope and is freed here.

        // reads data
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(5) {
            Some(six) => println!("The sixth element is {}", six),
            None => println!("There is no sixth element."),
        }

        // iterating vec
        for i in &mut v {
            *i += 1;
            println!(
                "now i is : {}",
                Red.bold().paint(format!("{}", *i)).to_string()
            );
        }

        use super::SpreadSheetCell as Ssc;
        let rows = vec![
            Ssc::Int(3),
            Ssc::Float(10.12),
            Ssc::Text(String::from("blue")),
        ];

        for row in &rows {
            println!(
                "{}",
                Red.italic()
                    .paint(format!("current row is {:?}", row))
                    .to_string()
            );
        }
    }

    #[test]
    fn test_string_api() {
        #[derive(Debug)]
        struct StringTester {
            pos: u32,
            msg: String,
            data: Vec<String>,
        }

        impl Display for StringTester {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "StringTester=> {{\"pos\":{},\"msg\":\"{}\",\"data\":{:?}}}",
                    &self.pos, &self.msg, &self.data
                )
            }
        }

        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
        let tester = StringTester {
            pos: 1,
            msg: "tester".to_string(),
            data: vec!["name".to_string(), "age".to_string(), "sex".to_string()],
        };
        println!("{}", tester.to_string());

        let s1 = String::from("hello, ");
        let s2 = String::from("world");
        let s3 = s1 + &s2;
        println!("{}", Colour::Blue.paint(s3));
    }

    #[test]
    fn test_hashmap_api() {
        let teams = vec!["Blue".to_string(), "Yellow".to_string()];
        let initial_scores = vec![10, 50];
        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
        let score = scores.entry(String::from("Blue")).or_insert(60);
        println!("score of Blue is : {}", score);

        // replace old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}

#[derive(Debug)]
pub enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
