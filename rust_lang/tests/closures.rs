//! Closures
#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|c| -c.population)
}

#[test]
fn test_closure() {
    let mut cities = Vec::new();
    cities.push(City {
        name: "Shanghai".to_string(),
        population: 2000_0000,
        country: "China".to_string(),
    });
    cities.push(City {
        name: "Beijing".to_string(),
        population: 1900_0000,
        country: "China".to_string(),
    });

    println!("cities: {:?}", &cities);

    sort_cities(&mut cities);
    println!("sorted cities by population: {:?}", &cities);
}

use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::time::Instant;
#[test]
fn test_threads() {
    let worker = |timeout, name: &'static str| {
        {
            let d = Duration::from_secs(timeout);
            thread::Builder::new()
                .name(name.to_string())
                .spawn(move || {
                    let start = Instant::now();
                    println!("thread-{} started at: {:?}", name, start);
                    thread::sleep(d);
                    println!("thread-{} ended at: {:?}", name, Instant::now());
                })
        }
        .unwrap()
    };

    println!("to start all");
    let start = Instant::now();
    let _ = worker(1, "1"); //.join();
    println!("start one");
    let _ = worker(2, "3"); //.join();
    println!("start two");
    let _ = worker(2, "4"); //.join();
    println!("start three");
    // threads start at join point.
    let _ = worker(3, "2").join();
    println!("time elapsed: {}", start.elapsed().as_secs());
}

/// Functions only
fn count_selected_cities_f(cities: &Vec<City>, predicate: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if predicate(city) {
            count += 1;
        }
    }

    return count;
}

/// Functions and Closures all supported;
fn count_selected_cities<F>(cities: &Vec<City>, predicate: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if predicate(city) {
            count += 1;
        }
    }

    return count;
}

fn test_selected_city(city: &City) -> bool {
    city.population > 1000_0000
}
#[test]
fn test_fn() {
    let mut cities = Vec::new();
    cities.push(City {
        name: "Shanghai".to_string(),
        population: 2000_0000,
        country: "China".to_string(),
    });
    cities.push(City {
        name: "Beijing".to_string(),
        population: 1900_0000,
        country: "China".to_string(),
    });

    let size = count_selected_cities(&cities, |city| city.population > 1000_0000);
    println!("selected size: {}", size);
    let size = count_selected_cities_f(&cities, test_selected_city);
    println!("selected size: {}", size);
}

#[test]
fn test_closure_clone() {
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Andrew");
    greet.clone()("Yaphets");
}

use std::collections::HashMap;
#[derive(Debug)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

use std::boxed::Box;
type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

/// Routers key: url
struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    /// Create an empty router.
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router,
    /// 'static to avoid closures contains borrowed references to variables
    /// that are about to go out of scope.
    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    /// a method to response
    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => self.not_found_response(),
            Some(callback) => callback(request),
        }
    }

    fn not_found_response(&self) -> Response {
        Response {
            code: 404,
            headers: HashMap::new(),
            body: "page not found".as_bytes().to_vec(),
        }
    }
}

fn check_sign(req: &Request) -> bool {
    println!("checking sign.");
    true
}

#[test]
fn test_handle_request() {
    let mut router = BasicRouter::new();
    router.add_route("/", |req| Response {
        code: 200,
        headers: HashMap::new(),
        body: "root page".as_bytes().to_vec(),
    });
    // let check_sign = |req| {
    //     print!("check sign");
    //     true
    // };
    router.add_route("/index", |req| {
        if check_sign(req) {
            println!("sign ok");
        }
        Response {
            code: 200,
            headers: HashMap::new(),
            body: "index page".as_bytes().to_vec(),
        }
    });

    println!("routers: {:?}", &router.routes.keys());
}

fn check_request<'a>(req: &'a Request, name: &'static str) -> &'a Request {
    println!("check request for: {}", name);
    let boxed = Box::new(req);
    println!("boxed: {:?}", boxed);
    req
}

const KEY: &'static str = "aoj29asf0daj";
/// error examples.
// fn check_closures(routes: &mut HashMap<String, BoxedCallback>, url: &'static str) {
//     // let key_ = "keytochecksign";
//     let is_sign_ok = |to_sign: &str, sign: &str, key: &str| {
//         println!("here check sign: {}, {}, {}", to_sign, sign, key);
//         true
//     };
//     let check_sign = |to_sign, sign| {
//         println!("to_sign: {}, sign: {}", to_sign, sign);
//         is_sign_ok(to_sign, sign, KEY)
//     };

//     let _check_sign = |req: &Request| {
//         if check_sign(req.method.as_str(), req.url.as_str()) {
//             println!("sign check success");
//         }
//         Response {
//             code: 200,
//             headers: HashMap::new(),
//             body: Vec::new(),
//         }
//     };

//     let route = Box::new(_check_sign);
//     routes.insert(url.to_string(), route);
// }

#[test]
fn test_box_static() {
    let req = Request {
        method: "GET".to_string(),
        url: "/".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let req = check_request(&req, "search for the users");
    let str = String::from("hello");
    {
        let boxed = Box::new(req);
        println!("{:?}", boxed);
    }

    // check_closures(&mut HashMap::new(), "/");
}
