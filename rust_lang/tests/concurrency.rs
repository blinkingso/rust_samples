//! concurrency programming.

struct Worker<F>
where
    F: FnOnce() -> (),
{
    id: u64,
    name: String,
    worker: F,
}

impl<F> Worker<F>
where
    F: FnOnce() -> (),
{
    fn work(self) {
        let start = time::Instant::now();
        println!(
            "start to do worker: {}-{} at {:?}",
            &self.id, &self.name, start
        );
        (self.worker)();
        println!(
            "worker: {}-{} finished with in {} seconds",
            &self.id,
            &self.name,
            start.elapsed().as_secs()
        );
    }
}

use std::fs::File;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::{fs, io, thread, time};

/// Fork-Join Parallelism. panic spreads.
#[test]
fn test_fork_join() -> io::Result<()> {
    let j = thread::spawn(|| {
        println!("start to run ");
        // panic!("sub thread panic test");
        let i = 10i32;
        let j = i.checked_div(1).unwrap();
        println!("j is {}", j);
        fs::File::open("/root/a.txt")
    });

    // unwrap failed here will cause panic
    let f = j.join().unwrap()?;

    println!("main thread");
    Ok(())
}

#[test]
fn test_channel() {
    let documents: Vec<PathBuf> = vec![];
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename);
            if let Some(text) = text.ok() {
                if tx.send(text).is_err() {
                    break;
                }
            }
        }
    });
}

use crate::shared_channel::shared_channel;
use crossbeam::epoch::pin;
use graphics::text;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::{channel, IntoIter};
use std::sync::{mpsc, Arc, Condvar, Mutex, RwLock};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

fn start_file_reader_thread(
    documents: Vec<PathBuf>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    let reader_file_handle = thread::spawn(move || {
        for filename in documents {
            let f = fs::read_to_string(filename)?;
            if sender.send(f).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, reader_file_handle)
}

struct InMemoryIndex {
    doc_id: usize,
    text: String,
}

impl InMemoryIndex {
    fn from_single_document(doc_id: usize, text: String) -> Self {
        InMemoryIndex { doc_id, text }
    }
}

fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);
            if sender.send(index).is_err() {
                break;
            }
        }
    });
    (receiver, handle)
}

fn start_in_memory_merge_thread(
    file_indexes: mpsc::Receiver<InMemoryIndex>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();

    for idx in file_indexes.recv() {}
    let handle = thread::spawn(move || {});

    (receiver, handle)
}

fn start_index_writer_thread(
    big_indexes: mpsc::Receiver<InMemoryIndex>,
    output_dir: &Path,
) -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    for idx in big_indexes.recv() {}
    let handle = thread::spawn(move || Ok(()));
    (receiver, handle)
}

/// Merge all files.
/// # Parameters
/// * files([`mpsc::Receiver<PathBuf>`]): Files Received from Channel
/// * output_dir([`Path`]): Dst file to write.
/// # Returns
/// io::Result<()>
fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path) -> io::Result<()> {
    Ok(())
}

fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_files(files, &output_dir);

    // wait for threads to finish.
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();

    // Return the first error encountered, if any.
    r1?;
    r4?;
    result
}

pub trait OffThreadExt: Iterator {
    /// Transform this iterator into an off-thread iterator:
    /// the `next()` calls happen on a separate worker thread, so the
    /// iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        receiver.into_iter()
    }
}

#[test]
fn test_thread_ext() {
    let mut documents: Vec<PathBuf> = Vec::new();
    documents.push(PathBuf::from("./tests/file.rs"));
    documents.push(PathBuf::from("./tests/concurrency.rs"));
    documents.push(PathBuf::from("./tests/closures.rs"));
    let c: Vec<InMemoryIndex> = documents
        .into_iter()
        .map(|file| fs::read_to_string(file).unwrap())
        .off_thread()
        .enumerate()
        .map(|(doc_id, text)| {
            // file text to InMemoryIndex
            InMemoryIndex::from_single_document(doc_id, text)
        })
        .off_thread()
        .collect();

    for idx in c {
        println!("idx: {}=> text: {}", idx.doc_id, idx.text);
    }
}

/// Shared Mutable State
type PlayerId = u32;
const GAME_SIZE: usize = 8;
/// A waiting list never grows to more than GAME_SIZE players.
type WaitingList = Vec<PlayerId>;

/// All threads have shared access to this big context struct.
struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

impl FernEmpireApp {
    /// Create an empty app.
    fn new_app() -> Arc<FernEmpireApp> {
        let app = Arc::new(FernEmpireApp {
            waiting_list: Mutex::new(vec![]),
        });

        app
    }

    fn join_waiting_list(&self, player: PlayerId) {
        // Lock the mutex and gain access to the data inside.
        let mut guard = self.waiting_list.lock().unwrap();

        // game logic
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            // free the lock before scope.
            drop(guard);
            self.start_game(players);
        }
    }

    fn start_game(&self, players: Vec<PlayerId>) {}
}

#[test]
fn test_panic_threads() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            if i % 2 == 0 {
                panic!("i = {}, panics here.", i);
            } else {
                thread::sleep(Duration::from_secs(1));
                println!("i = {}, sub threads is ok.", i);
            }
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}

pub mod shared_channel {
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::{Arc, Mutex};

    /// A thread-safe wrapper around a `Receiver`.
    #[derive(Clone)]
    pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

    impl<T> Iterator for SharedReceiver<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let guard = self.0.lock().unwrap();
            guard.recv().ok()
        }
    }

    pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
        let (sender, receiver) = channel();
        (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
    }
}

#[test]
fn test_shared_channel() {
    let (sender, shared_receiver) = shared_channel();
    let s_handle = thread::spawn(move || {
        for i in 0..500000 {
            if sender.send(format!("msg-{}", i)).is_err() {
                break;
            }
        }
    });
}

#[test]
fn test_rwlock() {
    let a = 1024;
    let rw = RwLock::new(a);
    {
        let a1 = rw.read().unwrap();
        let a2 = rw.read().unwrap();
        let a3 = rw.read().unwrap();
        println!("read locks can exits: {}, {}, {}", *a1, *a2, *a3);
    }
    let mut wa = rw.write().unwrap();
    *wa += 10;
    println!("{}", wa);
}

#[test]
fn test_condition_variant() {
    let cond: Condvar = Condvar::new();
    let mut mutex = Arc::new(Mutex::new(false));
    let lock = &*mutex;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cond.wait(started).unwrap();
    }
}

#[test]
fn test_atomic() {
    let i = AtomicIsize::new(0);
    i.fetch_add(1, Ordering::SeqCst);
    println!("i is : {}", i.load(Ordering::SeqCst));
}
