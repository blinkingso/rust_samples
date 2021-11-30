//! Chapter 13. Utiliy Traits;
//!
//! Drop 析构函数,
//! Sized 标记trait, 编译时期固定内存占用大小
//! Clone Copy 克隆值, Copy->标记trait, 在内存中按字节进行拷贝
//! Deref DerefMut 智能指针 解引用.
//! Default 给定默认值
//! AsRef AsMut 借引用
//! Borrow BorrowMut 类似AsRef,AsMut, 新增hash, ordering, equality的功能(即Java中的,equals hash compable能力)
//! From Into 类型数据之间相互转换, 比如字符串转数字类型等.
//! TryFrom TryInto 转换可能失败
//! ToOwned 引用->拥有值(获得所有权)
//!

use std::{
    collections::hash_map::DefaultHasher,
    ffi::{OsStr, OsString},
    fmt::Display,
    hash::{Hash, Hasher},
    io::{ErrorKind, Read},
    ops::{Deref, DerefMut},
    os::unix::prelude::OsStrExt,
    path::Path,
};

use graphics::math::Vec2d;
struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

#[test]
fn test_drop() {
    let mut a = Appellation {
        name: "zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };
    println!("before assignment");
    a = Appellation {
        name: "hera".to_string(),
        nicknames: vec![],
    };

    println!("at end of block");

    let p;
    {
        let q = Appellation {
            name: "Cardamime".to_string(),
            nicknames: vec!["shotweed".to_string(), "bittercress".to_string()],
        };

        if false {
            p = q;
        }
    }

    println!("Sproing! what wat that?");
}

/// Sized , Sized类型的值在内存中占用相同的空间大小. Rust中的类型大都是Sized类型
/// u64 (8bytes), (f32, f32, f32) tuple(12bytes) enums are sized too.
/// Vec<T>指向内存中的buffer的指针, 包括了lenth和capacity. Vec<T>也是Sized类型的.指针size + len + capacity.
/// Marker traits.
/// UnSized类型: str
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<dyn std::fmt::Display>) {
    println!("{}", &boxed.value);
}

#[test]
fn test_sized() {
    let b = RcBox {
        ref_count: 1,
        value: "launch".to_string(),
    };
    let boxed_displayable: &RcBox<dyn std::fmt::Display> = &b;
    println!("{}", &boxed_displayable.value);
    display(&boxed_displayable);
}

/// Clone must be infallible,
/// 克隆操作不能出现错误, 比如文件的克隆可能会失败, 所以Rust的File没有实现Clone trait.
/// 操作系统可能资源不够用, 可以使用TryClone trait来进行clone, 返回Result<File>来判断文件克隆是否成功.
/// 例如大文件的克隆可能因为磁盘空间不足等引发错误.
#[test]
fn test_clone() {
    let s = String::from("hello world");
    let mut m = String::new();
    m.clone_from(&s);
}

/// Copy trait
/// = 绑定操作一般触发所有权的move.  简单数据类型实现了Copy trait, 在=值时完成了栈内值拷贝. 不存在move.

/// Deref DerefMut * . behave
struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}

fn show_it(s: &str) {
    println!("{}", s);
}

#[test]
fn test_deref() {
    let mut selector = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 1,
    };
    assert_eq!(*selector, 'y');
    assert!(selector.is_alphabetic());
    *selector = 'w';
    assert_eq!(selector.elements, ['x', 'w', 'z']);
    let selector = Selector {
        elements: vec!["good".to_string(), "bad".to_string(), "ugly".to_string()],
        current: 1,
    };
    show_it_generic(&*selector);
    show_it(&selector);
}

struct FilePath {
    root: String,
    file_name: String,
    suffix: String,
}

// impl AsRef<OsStr> for FilePath {
//     fn as_ref(&self) -> &OsStr {
//         let msg = format!("{}/{}.{}", self.root, self.file_name, self.suffix);
//         let s = msg.as_bytes();
//         println!("file_name is : {}", &msg);
//         unsafe { &*(s as *const [u8] as *const OsStr) }
//     }
// }

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        let msg = format!("{}/{}.{}", self.root, self.file_name, self.suffix);
        let s = msg.as_bytes();
        println!("file_name is : {}", &msg);
        unsafe {
            let path = &*(s as *const [u8] as *const OsStr);
            &Path::new(path)
        }
    }
}

impl AsMut<Path> for FilePath {
    fn as_mut(&mut self) -> &mut Path {
        let msg = format!("{}/{}.{}", self.root, self.file_name, self.suffix);
        let s = msg.as_bytes();
        println!("file_name is : {}", &msg);
        unsafe {
            let path = &*(s as *const [u8] as *const OsStr);
            let path = Path::new(path);
            &mut *(path as *const Path as *mut Path)
        }
    }
}

/// AsRef AsMut
#[test]
fn test_as_ref() {
    let f = FilePath {
        root: "/Users/andrew/CLionProjects/rust_samples/rust_lang/tests".to_string(),
        file_name: "traits_genrics".to_string(),
        suffix: "rs".to_string(),
    };
    // let path = (f.as_ref() as &OsStr).as_ref() as &Path;
    let mut f = std::fs::File::open(&f).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    println!("file contents \n {}", contents);
}

/// Hash Eq
#[derive(Debug, PartialEq, Eq)]
struct User {
    name: String,
    age: u8,
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

use std::collections::HashMap;
#[test]
fn test_hash_eq() {
    let mut data = HashMap::new();
    data.insert(
        User {
            name: "andrew".to_string(),
            age: 20,
        },
        "Java Serials Course",
    );

    println!("data: {:?}", data);
}

use std::io::Error;
fn des(error: &Error) -> &'static str {
    match error.kind() {
        ErrorKind::OutOfMemory => "OutOfMemory",
        ErrorKind::AddrInUse => "AddrInUse",
        _ => "Unknown Error",
    }
}

#[test]
fn test_match() {
    let error = std::io::Error::new(ErrorKind::OutOfMemory, "OOM");
    let s1 = des(&error);
    println!("s1 is : {}", s1);
    std::mem::drop(error);
}

use std::borrow::Cow;
#[test]
fn test_cow() {
    let name = "yaphets";
    let name1 = String::from("andrew");
    let name2 = Cow::Borrowed(name);
    // name1 remove after here.
    let name3 = Cow::<'_, String>::Owned(name1);

    println!("{}, {}, {}, {}", name, name, name2, name3);
    // name2 value borrowed here after move;
    let name4 = name2.into_owned();
    // println!("{}, {}, {}, {}", name, name4, name2, name3);
    println!("{}, {}, {}", name, name4, name3);
}
