use std::{collections::HashMap, path::Path};

use libactionkv::ActionKV;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "akv_disk",
    about = "An In-Memory database base on Local disk storage!"
)]
pub struct CommandOpt {
    /// A dest file to store data in bytes
    #[structopt(short, long = "file", default_value = "akv_disk.dib")]
    pub file_name: String,
    /// SubCommands to support Insert, Update, Get, Delete operations
    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Insert new Key-Value to the db
    Insert {
        /// Key to insert
        #[structopt(short, long)]
        key: String,
        /// Value to insert
        #[structopt(short, long)]
        value: String,
    },
    /// Delete the key value
    Delete {
        /// Key to delete
        #[structopt(short, long)]
        key: String,
    },
    /// Get value from the key
    Get {
        /// Key to get
        #[structopt(short, long)]
        key: String,
        /// Whether or not to scan the Local Storage
        #[structopt(short, long)]
        scan: bool,
    },
    /// Find key from memory or Local Disk
    Find {
        /// Key to find
        #[structopt(short, long)]
        key: String,
    },
    /// Update exist key value
    Update {
        /// Key to set
        #[structopt(short, long)]
        key: String,
        /// Value to set
        #[structopt(short, long)]
        value: String,
    },
}
type ByteStr = [u8];
type ByteString = Vec<u8>;

const INDEX_KEY: &'static ByteStr = b"+index";
fn main() -> std::io::Result<()> {
    let commands = CommandOpt::from_args();
    let subcommand = commands.cmd;
    let file = commands.file_name;
    let path = Path::new(&file);
    let mut store = ActionKV::open(path).expect("Unable to open file");
    // load all data to the memory
    store.load()?;
    match subcommand {
        SubCommand::Insert { key: k, value: v } => {
            if let Ok(_) = store.insert(&k.as_bytes(), &v.as_bytes()) {
                store.store_index_on_disk(&INDEX_KEY);
                print("ok");
            } else {
                print("insert failed.");
            }
        }
        SubCommand::Get { key: k, scan } => {
            // load memory
            let index_as_bytes = store.get(&INDEX_KEY, scan)?.unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();
            match index.get(&k.as_bytes().to_vec()) {
                None => eprint(&format!("{} not found.", &k)),
                Some(&i) => {
                    let kv = store.get_at(i)?;
                    print(&String::from_utf8_lossy(&kv.value));
                }
            }
            // if let Some(v) = store.get(&k.as_bytes(), scan)? {
            //     print(&String::from_utf8_lossy(&v.as_slice()));
            // } else {
            //     eprint(&format!("{} not found.", &k));
            // }
        }
        SubCommand::Find { key } => {
            if let Some(v) = store.find(&key.as_bytes())? {
                let position = v.0;
                let value = String::from_utf8_lossy(&v.1);
                print(&format!("pos: {}, value: {}", position, value));
            } else {
                print(&format!("{} not found", &key));
            }
        }
        SubCommand::Update { key, value } => {
            if let Ok(_) = store.update(&key.as_bytes(), &value.as_bytes()) {
                store.store_index_on_disk(&INDEX_KEY);
                print("ok");
            } else {
                eprint("update failed.");
            }
        }
        SubCommand::Delete { key } => {
            if let Ok(_) = store.delete(&key.as_bytes()) {
                print("ok");
            } else {
                eprint("delete failed.");
            }
        }
    }
    Ok(())
}

pub fn print(msg: &str) {
    println!("> {}", &msg);
}
pub fn eprint(msg: &str) {
    eprintln!("> {}", &msg);
}
