use ansi_term::Colour::Red;
use std::env;

const USAGE: &str = "===>usage: minigrep [query string] [filename]";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let search = minigrep::Search::new(&args).unwrap_or_else(|e| {
        println!(
            "{}{}",
            e,
            Red.bold().italic().strikethrough().paint(USAGE).to_string()
        );

        std::process::exit(1);
    });

    println!(
        "{}",
        Red.bold()
            .italic()
            .paint("Search Result is >>>")
            .to_string()
    );
    let _ = minigrep::run(&search).unwrap_or_else(|e| {
        println!(
            "{}",
            Red.bold()
                .italic()
                .strikethrough()
                .paint(format!("{}", e))
                .to_string()
        );
        std::process::exit(1);
    });
}

pub(crate) mod minigrep {
    use ansi_term::Colour::Red;
    use std::error::Error;
    use std::fs::File;
    use std::io::{ErrorKind, Read};
    use std::path::Path;

    pub(crate) fn run(search: &Search) -> Result<(), Box<dyn Error>> {
        let file = File::open(Path::new(search.path))?;
        let metadata = file.metadata()?;
        if metadata.is_file() {
            // directly read from file.
            grep_file(search.query, search.path, search.case_sensitive);
        } else {
            // file is a dir
            grep_dirs(search.query, search.path, search.case_sensitive);
        }

        Ok(())
    }

    fn grep_dirs(query: &str, path: &str, case_insensitive: bool) {
        let parent_dir = Path::new(path);
        for entry in parent_dir.read_dir().expect("dir read failed...") {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        grep_file(query, entry.path().to_str().unwrap(), case_insensitive);
                    } else {
                        // read from dir recursive;
                        grep_dirs(query, entry.path().to_str().unwrap(), case_insensitive);
                    }
                }
            }
        }
    }

    pub fn grep_file<'a>(query: &'a str, path: &'a str, case_insensitive: bool) {
        let mut contents = String::new();
        let _ = File::open(&Path::new(path))
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        // search and println
        let lines = search(query, path, case_insensitive, &contents);
        for line in lines {
            line.print_line();
        }
    }

    pub fn search<'a, 'b>(
        query: &'a str,
        path: &'a str,
        case_insensitive: bool,
        contents: &'b str,
    ) -> Vec<LinedStr<'a, 'b>> {
        // let mut lines = Vec::new();
        let mut num = 0;
        contents
            .lines()
            .map(|line| {
                num += 1;
                (line, num)
            })
            .filter(|line| {
                return if case_insensitive {
                    line.0.contains(query)
                } else {
                    line.0.to_lowercase().contains(&query.to_lowercase())
                };
            })
            .map(|line| LinedStr {
                query,
                path,
                line: line.0,
                line_num: line.1,
                case_insensitive,
            })
            .collect()
    }

    pub struct Search<'a> {
        query: &'a str,
        path: &'a str,
        case_sensitive: bool,
    }

    use std::io::Error as IOError;

    impl<'a> Search<'a> {
        pub fn new(config: &'a Vec<String>) -> Result<Self, Box<dyn Error>> {
            // check
            if config.len() != 3 {
                return Err(Box::new(IOError::new(
                    ErrorKind::InvalidInput,
                    "command args not valid",
                )));
            }

            let query = &config[1];
            let path = &config[2];

            // check args;
            let _ = File::open(path)
                .map_err(|_| Box::new(IOError::new(ErrorKind::NotFound, "file not exists")))?;

            // read sensitive from env
            let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

            Ok(Search {
                query,
                path,
                case_sensitive,
            })
        }
    }

    pub struct LinedStr<'a, 'b> {
        query: &'a str,
        path: &'a str,
        line: &'b str,
        line_num: usize,
        case_insensitive: bool,
    }

    impl<'a, 'b> LinedStr<'a, 'b> {
        pub(crate) fn print_line(&self) {
            let line = self.line.trim();
            let ansi_str = Red.bold().paint(self.query).to_string();
            let line = line.replace(self.query, &ansi_str);
            let line = format!("{}::{}:==>{}", self.path, self.line_num, line);
            println!("{}", line);
        }
    }
}
