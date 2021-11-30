//! Concurrency Programming.
#[macro_use]
extern crate crossbeam;
use std::{fmt::Display, thread, time};

use crossbeam::channel::unbounded;
use svg::{
    node::element::{
        path::{Command, Data, Position},
        Path, Rectangle,
    },
    Document,
};
fn main() {
    let start = time::Instant::now();
    // Duration impl Clone, Copy trait.
    let pause = time::Duration::from_millis(300);

    let handler = thread::spawn(move || {
        thread::sleep(pause);
    });

    let handler2 = thread::spawn(move || {
        thread::sleep(pause);
    });

    // wait for handler finish
    handler.join().unwrap();
    println!(
        "elapsed time : {:02?}",
        time::Instant::now().duration_since(start)
    );
    handler2.join().unwrap();
    let finish = time::Instant::now();
    println!("{:02?}", finish.duration_since(start));
}

#[test]
fn test_1000_threads() {
    for n in 1..1001 {
        let mut handlers = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            let _ = handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

#[test]
fn test_1000_threads_2() {
    for n in 1..1001 {
        let mut handlers = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            let _ = handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_anonymous_func() {
        let add = |a: i32, b: i32| a + b;
        let add = |a, b| a + b;
        // println!("{}", add(1, 2));
        println!("{}", add("str".to_string(), "ing"));
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// render hex
#[derive(Debug)]
struct RenderHex {
    input: String,
    path: String,
}
use rayon::prelude::*;
impl RenderHex {
    fn new(input: &str) -> Self {
        let path = format!("{}.svg", input);
        RenderHex {
            input: String::from(input),
            path,
        }
    }

    fn with_path(input: &str, path: &str) -> Self {
        RenderHex {
            input: String::from(input),
            path: format!("{}.svg", &path),
        }
    }

    fn parse(&self) -> Vec<Operation> {
        let parse_byte = |byte: u8| {
            match byte {
                b'0' => Operation::Home,
                b'1'..=b'9' => {
                    // ASCII numerals start at 0x30(48)
                    let distance = (byte - 0x30) as isize;
                    Operation::Forward(distance * (HEIGHT / 10))
                }
                b'a' | b'b' | b'c' => Operation::TurnLeft,
                b'd' | b'e' | b'f' => Operation::TurnRight,
                _ => Operation::Noop(byte as usize),
            }
        };
        // self.input
        //     .as_bytes()
        //     .par_iter()
        //     .map(|byte| parse_byte(*byte))
        //     .collect()
        // mutlithreads to parse.
        let threads = 2;
        let (todo_tx, todo_rx) = unbounded();
        let (result_tx, result_rx) = unbounded();
        let mut n_bytes = 0;
        use crate::Work;
        for (i, byte) in self.input.bytes().enumerate() {
            // send to-parse bytes to the todo-channel
            todo_tx.send(Work::Task((i, byte))).unwrap();
            n_bytes += 1;
        }

        for _ in 0..threads {
            // send `Finished` Signal to help consumer to `break loop`
            todo_tx.send(Work::Finished).unwrap();
        }

        for _ in 0..threads {
            let todo = todo_rx.clone();
            let results = result_tx.clone();
            thread::spawn(move || loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((i, byte))) => (i, parse_byte(byte)),
                };
                results.send(result).unwrap();
            });
        }

        // recv parsed bytes from result_rx
        let mut ops = vec![Operation::Noop(0); n_bytes];
        for _ in 0..n_bytes {
            let (i, op) = result_rx.recv().unwrap();
            ops[i] = op;
        }

        ops
    }

    fn convert(&self) -> Vec<Command> {
        let operations = self.parse();
        let mut turtle = Artist::new();
        let mut path_data = vec![];
        let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
        path_data.push(start_at_home);
        for op in operations {
            match op {
                Operation::Forward(distance) => turtle.forward(distance),
                Operation::TurnLeft => turtle.turn_left(),
                Operation::TurnRight => turtle.turn_right(),
                Operation::Home => turtle.home(),
                Operation::Noop(byte) => {
                    // eprintln!("warning: illegal byte encountered: {:?}", byte)
                }
            }

            let line = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
            path_data.push(line);
            turtle.wrap();
        }

        path_data
    }

    fn generate_svg(&self, path_data: Vec<Command>) -> Document {
        println!("path data is : {:?}", &path_data);
        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", WIDTH)
            .set("height", HEIGHT)
            .set("fill", "#ffffff");
        let border = background
            .clone()
            .set("fill-opacity", "0.0")
            .set("stroke", "#cccccc")
            .set("stroke-width", 3 * STROKE_WIDTH);
        let sketch = Path::new()
            .set("fill", "none")
            .set("stroke", "#2f2f2f")
            .set("stroke-width", STROKE_WIDTH)
            .set("stroke-opacity", "0.9")
            .set("d", Data::from(path_data));
        let document = Document::new()
            .set("viewBox", (0, 0, HEIGHT, WIDTH))
            .set("height", HEIGHT)
            .set("width", WIDTH)
            .set("style", r#"style="outline: 5px solid #800000;""#)
            .add(background)
            .add(sketch)
            .add(border);

        document
    }

    fn save(&self) {}
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
}

#[derive(Debug, Clone)]
enum Orientation {
    East,
    South,
    West,
    North,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direct = match self {
            Orientation::East => "East",
            Orientation::South => "South",
            Orientation::West => "West",
            Orientation::North => "North",
        };
        write!(f, "{}", direct)
    }
}
const WIDTH: isize = 400;
const HEIGHT: isize = 400;
const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;
const STROKE_WIDTH: usize = 5;
#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Self {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: Orientation::North,
        }
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::East => self.x += distance,
            Orientation::South => self.y -= distance,
            Orientation::West => self.x -= distance,
            Orientation::North => self.y += distance,
        };
        println!(
            "go straight forward to {} with {}m",
            &self.heading, distance
        );
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn turn_left(&mut self) {
        let now_forward = self.heading.clone();
        self.heading = match self.heading {
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::North => Orientation::West,
        };
        println!("Turn left from {} to {}", &now_forward, &self.heading);
    }

    fn turn_right(&mut self) {
        let now_forward = self.heading.clone();
        self.heading = match self.heading {
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::North => Orientation::East,
        };
        println!("Turn left from {} to {}", &now_forward, &self.heading);
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South;
        }
    }
}

#[test]
fn test_draw_svg() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = args.get(2).unwrap();
    let path = String::from(input);
    let path = args.get(3).unwrap_or(&path);
    let render = RenderHex::with_path(input, path);
    println!("render is : {:?}", render);
    let path_data = render.convert();
    let document = render.generate_svg(path_data);
    svg::save(&render.path, &document).unwrap();
}

#[test]
fn test_unbounded() {
    let (tx, rx) = crossbeam::channel::unbounded();
    std::thread::spawn(move || {
        tx.send(42).unwrap();
    });

    // rec(rx) is syntax defined by the macro.
    select! {
        recv(rx) -> msg => println!("{:?}", msg),
    }
}

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

#[test]
fn test_ping_pong() {
    use crate::ConnectivityCheck::*;
    let n_messages = 3;
    let (req_tx, req_rx) = unbounded();
    let (res_tx, res_rx) = unbounded();
    thread::spawn(move || loop {
        match req_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => res_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        req_tx.send(Ping).unwrap();
    }

    req_tx.send(Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv(res_rx) -> msg => println!("{:?}", msg),
        }
    }
}

enum Work {
    Task((usize, u8)),
    Finished,
}
