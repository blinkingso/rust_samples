//! fs
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{
    canonicalize, copy, create_dir, create_dir_all, metadata, remove_dir, remove_dir_all,
    set_permissions, File, FileType, OpenOptions, Permissions,
};
use std::io::{BufReader, ErrorKind, Read, Seek, SeekFrom, Write};
#[cfg(unix)]
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

type NacosConfResult = std::io::Result<HashMap<String, String>>;

/// to let our program cross platform.
#[cfg(not(unix))]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("can't copy symbolic link: {}", original.as_ref().display()),
    ))
}
#[test]
fn test_path() {
    let f = Path::new("./closures.rs");
    display_path(f);
    let f = Path::new("../tests/file.rs");
    display_path(f);
    let f = Path::new("/Users/andrew/CLionProjects/rust_samples/rust_lang/tests/file.rs");
    display_path(f);
    println!("file path: {}", f.display());
}

fn create_file<P>(p: P) -> Option<File>
where
    P: AsRef<Path>,
{
    let f = File::create(&p);
    match f {
        Ok(f) => Some(f),
        Err(err) => {
            println!("File {} create error for {:?}", p.as_ref().display(), err);
            None
        }
    }
}

fn add_conf(data_center: &str, group_id: &str, key_name: &str, value: &str) -> std::io::Result<()> {
    let line_key = format!("group_id::{}", group_id);
    let mut text_map = read_conf(data_center)?;
    let mut d = read_line(data_center, group_id)?;
    d.insert(key_name.to_string(), value.to_string());
    text_map.insert(line_key, serde_json::to_string(&d)?);
    write_conf(data_center, group_id, key_name, value, text_map)
}

fn get_conf(data_center: &str, group_id: &str, key_name: &str) -> Option<String> {
    let line = read_line(data_center, group_id).ok()?;
    Some(line.get(key_name)?.to_string())
}

fn write_conf(
    data_center: &str,
    group_id: &str,
    key_name: &str,
    value: &str,
    text_map: HashMap<String, String>,
) -> std::io::Result<()> {
    let mut result = String::new();
    text_map.iter().for_each(|entry| {
        let line = String::from(entry.0) + "=" + entry.1 + LF;
        result.push_str(line.as_str());
    });
    let mut f = open_file(data_center)?;
    f.seek(SeekFrom::Start(0))?;
    f.write_all(result.as_bytes())?;
    f.flush()
}

fn open_file(data_center: &str) -> std::io::Result<File> {
    let path = format!("{}{}.properties", FILE_NAME_PREFIX, data_center);
    let f = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(false)
        .truncate(false)
        .open(path.as_str());
    match f {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("File NotFound, may create your dir.");
                let p = Path::new(&path);
                if let Some(parent) = p.parent() {
                    create_dir(parent)?;
                    println!("create dir ok.");
                    return open_file(data_center.clone());
                }
                Err(error)
            }
            ErrorKind::PermissionDenied => {
                eprintln!("PermissionDenied to create the file.");
                Err(error)
            }
            _ => {
                eprintln!("unknown error: {:?}", error);
                Err(error)
            }
        },
    }
}

fn read_line(data_center: &str, group_id: &str) -> NacosConfResult {
    let line_key = format!("group_id::{}", group_id);
    let text_map = read_conf(data_center)?;
    Ok(if let Some(json_str) = text_map.get(line_key.as_str()) {
        serde_json::from_str(json_str.as_str())?
    } else {
        HashMap::new()
    })
}

const FILE_NAME_PREFIX: &'static str = "./conf/nacos-";
const EQ: &'static str = "=";
const LF: &'static str = "\r\n";
fn read_conf(data_center: &str) -> NacosConfResult {
    let f = open_file(data_center)?;
    read_conf_file(&f)
}
fn read_conf_file(f: &File) -> NacosConfResult {
    let mut text = String::new();
    let mut buf = BufReader::new(f);
    let _ = buf.read_to_string(&mut text)?;
    let result = text
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let mut ls = line.splitn(2, EQ);
                Some((
                    ls.next().unwrap().to_string(),
                    ls.next().unwrap().to_string(),
                ))
            }
        })
        .collect();
    Ok(result)
}

fn display_path(path: &Path) {
    println!("{:=^25}", "*****");
    path.ancestors().for_each(|p| {
        println!("{:?}", p.to_str());
    });
}

#[allow(dead_code)]
fn process_error(error: &str) -> Cow<'static, str> {
    match error {
        "io" => Cow::Owned(String::from(error)),
        _ => Cow::Borrowed("unknown error"),
    }
}

#[test]
fn test_add_conf() -> std::io::Result<()> {
    let data_center = "pay";
    let group_id = "yz";
    add_conf(data_center, group_id, "gateway", "http://baidu.com")?;
    add_conf(data_center, group_id, "gateway1", "http://baidu1.com")?;
    add_conf(data_center, group_id, "gateway2", "http://baidu2.com")?;
    add_conf(data_center, group_id, "gateway3", "http://baidu3.com")?;
    add_conf(data_center, group_id, "gateway", "http://baidu4.com")?;
    add_conf(data_center, "yz2", "gateway", "http://baidu4.com")?;
    add_conf(data_center, "yz3", "gateway", "http://baidu4.com")?;
    let v = get_conf(data_center, "yz", "gateway");
    println!("got: {} = {}", "gateway", v.unwrap());
    Ok(())
}

#[test]
fn test_file_functions() {
    // create_dir mkdir
    create_dir("./conf").unwrap_or_else(|e| eprintln!("create dir error: {:?}", e));
    // create_dir_all == > mkdir -p
    create_dir_all("./conf/nacos/json").unwrap_or_else(|e| {
        eprintln!("create dir error for: {:?}", e);
    });
    // remove_dir ==> rmdir
    remove_dir("./conf/nacos/json").unwrap_or_else(|e| eprintln!("rm dir error for : {:?}", e));
    // rm -r
    remove_dir_all("./conf/nacos").unwrap_or_else(|e| eprintln!("rm -r error for : {:?}", e));
    // cp -p
    let _ = copy("./tests/file.rs", "./tests/file.rs.1").unwrap_or_else(|e| {
        eprintln!("copy file error for {:?}", e);
        0
    });
    let path = canonicalize("./tests/file.rs").unwrap();
    println!("real path is : {}", path.display());
    let m = metadata("./tests/closures.rs").unwrap_or_else(|e| {
        eprintln!("metadata get error: {:?}", e);
        panic!("error read metadata: {:?}", e);
    });
    println!("metadata: {:?}", m);
    // let _ = set_permissions("./tests/file.rs", Permissions::from_mode(755));
}
#[test]
fn test_dir() -> std::io::Result<()> {
    for entry in Path::new("/Users/andrew").read_dir()? {
        let entry = entry?;
        // println!("{}", entry.file_name().to_string_lossy());
        println!("{}", entry.path().display());
    }

    Ok(())
}
use std::io;
use std::net::TcpListener;

fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        create_dir(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}
fn copy_to(src: &Path, src_type: &FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        // copy(src, dst)?;
    } else if src_type.is_dir() {
        println!(
            "src path: {}, file name: {}, dst path: {}",
            &src.display(),
            &src.file_name().unwrap().to_string_lossy(),
            &dst.display()
        );
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.read_link()?;
        symlink(target, dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("don't know how to copy: {}", src.display()),
        ));
    }
    Ok(())
}

#[test]
fn test_copy_dir() -> io::Result<()> {
    let src = Path::new("/Users/andrew/CLionProjects/rust_samples");
    let dst = Path::new("/Users/andrew/CLionProjects/rust_samples2");
    copy_dir_to(&src, &dst)?;
    remove_dir_all(dst)?;
    Ok(())
}

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        let (mut socket, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        // spawn a thread to handle this client.
        let mut out = socket.try_clone()?;
        std::thread::spawn(move || {
            io::copy(&mut socket, &mut out).expect("error in client thread: ");
            println!("connection closed.");
        });
    }
}

#[test]
fn test_tcp() {
    echo_main("127.0.0.1:17000").expect("error:")
}
