//! Asynchronous Programming.

use std::error::Error;
use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{io, net};

fn cheapo_request(host: &str, port: u16, path: &str) -> io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port))?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

#[test]
fn test_baidu() {
    cheapo_request("baidu.com", 80, "/").ok().take().map(|res| {
        println!("{}", res);
    });
}

#[test]
fn test_async_fn() {
    use std::future::Future;
}

#[derive(Clone)]
struct ReadToString(String);

impl Future for ReadToString {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl ReadToString {
    fn read_to_string(&mut self, buf: &mut String) -> impl Future<Output = io::Result<usize>> {
        ReadToString(buf.clone())
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

struct T<E: 'static> {
    data: E,
}

impl<E: 'static> T<E> {
    fn new(data: E) -> T<E> {
        T { data }
    }
}

#[test]
fn test_t() {
    let s = String::from("hello world");
    let t = T::new(s);
    println!("{}", t.data);
    // println!("s has been moved before: {}", s);
}

/// Long running Computations: yield_now and spawn_blocking.
async fn long_computation() {}

async fn verify_password(password: &str, hash: &str, key: &str) -> Result<bool, String> {
    let password = password.to_string();
    let hash = hash.to_string();
    let key = key.to_string();

    async_std::task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
            .map_err(|e| {
                eprintln!("verify error here for: {:?}", e);
                "password verify error".to_string()
            })
    })
    .await
}

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Error>> {
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

#[test]
fn test_many_requests() {
    let requests = &[
        "http://baidu.com".to_string(),
        "http://google.com".to_string(),
        "https://baidu.com".to_string(),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("***{}\r\n", response),
            Err(err) => eprintln!("error : {} \r\n", err),
        }
    }
}
