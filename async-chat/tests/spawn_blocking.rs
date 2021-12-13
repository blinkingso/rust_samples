//! simple spawn_blocking for Future.

use std::future::Future;
use std::marker::PhantomPinned;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Instant;

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

/// Poll::Ready(T)
struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

impl<T: Send> Future for SpawnBlocking<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            println!("task has been finished");
            return Poll::Ready(value);
        }

        guard.waker = Some(cx.waker().clone());
        println!("task is pending now...");
        Poll::Pending
    }
}

pub fn spawn_blocking<T, F>(closure: F) -> SpawnBlocking<T>
where
    F: FnOnce() -> T,
    T: Send + 'static,
    F: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    std::thread::spawn({
        let inner = inner.clone();
        move || {
            let value = closure();
            let maybe_waker = {
                let mut guard = inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };
            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });

    SpawnBlocking(inner)
}

use crossbeam::sync::Parker;
use futures_lite::pin;
use waker_fn::waker_fn;
fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);
    // to a Pin<&mut F>
    // pin!(future);
    let mut future = future;
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

async fn long_async_fn() {
    let start = Instant::now();
    println!("long start");
    let mut i = 0;
    loop {
        if i == i32::MAX / 4 {
            break;
        }
        i += 1;
    }
    println!("long end with: {} mills", start.elapsed().as_millis());
}

async fn short_async_fn() {
    println!("short async start");
    println!("short async end");
}

#[test]
fn test_block_async() {
    block_on(async {
        let start = Instant::now();
        long_async_fn().await;
        long_async_fn().await;
        short_async_fn().await;
        println!("async elapsed: {} mills", start.elapsed().as_millis());
    });

    println!("=======================");

    async_std::task::block_on(async {
        let start = Instant::now();
        async_std::task::block_on(async {
            long_async_fn().await;
        });
        async_std::task::block_on(async {
            long_async_fn().await;
        });
        short_async_fn().await;
        println!("async elapsed: {} mills", start.elapsed().as_millis());
    });
}

#[test]
fn test_deref_mut() {
    let s = String::from("hello");
    let mut b = Box::new(s);
    b.deref_mut().push_str("hello world");
    println!("now b is : {}", *b);
}

struct FixedBox<T: Sized> {
    data: T,
}

impl<T: Sized> FixedBox<T> {
    fn size_of(&self) -> usize {
        std::mem::size_of_val(&self.data)
    }
}

#[test]
fn test_sized() {
    let vectors = vec!["hello", "world"];
    // [u8]: unsized here.
    // let vectors = vectors.as_slice()[..];
    let f = FixedBox { data: vectors };
    println!("data size: {}", f.size_of());

    {
        trait Foo {}
        trait Bar: Sized {}
        struct Impl;
        impl Foo for Impl {}
        impl Bar for Impl {}

        let x: &dyn Foo = &Impl;
        // here compile error for: cannot be made into an object.
        // Bar: Sized which means trait object is Sized is error.
        // let y: &dyn Bar = &Impl;
        println!("any compile error here?");
    }
}

#[test]
fn test_trait_objects() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        fn run(&self) {
            for c in &self.components {
                c.draw();
            }

            println!("c.len() is : {}", self.components.len());
        }
    }

    struct Button;
    struct CheckBox;
    struct SelectBox;
    impl Draw for Button {
        fn draw(&self) {
            println!("Button");
        }
    }
    impl Draw for CheckBox {
        fn draw(&self) {
            println!("CheckBox");
        }
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox");
        }
    }

    let screen = Screen {
        components: vec![Box::new(SelectBox), Box::new(Button), Box::new(CheckBox)],
    };

    screen.run();

    fn draws(c1: &dyn Draw, c2: &dyn Draw) {
        c1.draw();
        c2.draw();
    }

    draws(&Button, &CheckBox);

    /// c1, c2 concrete type must be T;
    fn draw2<T: Draw>(c1: &T, c2: &T) {
        c1.draw();
        c2.draw();
    }

    // compile error here for c1, c2 concrete types are not the same.
    // draw2(&Button, &CheckBox);

    fn draw3(c1: &impl Draw, c2: &dyn Draw) {
        c1.draw();
        c2.draw();
    }

    // its ok here
    draw3(&Button, &CheckBox);

    fn draw4<T>(c1: &Box<T>, c2: &Box<T>)
    where
        T: Draw,
    {
        c1.deref().draw();
        c2.deref().draw();
    }

    // compile error here for T not the same concrete type.
    // draw4(&Box::new(Button), &Box::new(SelectBox));

    // struct CloneOption {
    //     clones: Vec<Box<dyn Clone>>,
    // }

    /// not object-safe. with trait-object.
    trait Cq {
        fn cq(&self, rhs: &Self) -> bool;
    }

    impl Cq for CheckBox {
        fn cq(&self, _: &Self) -> bool {
            false
        }
    }

    impl Cq for Button {
        fn cq(&self, rhs: &Self) -> bool {
            true
        }
    }

    // fn cq_tester(lhs: &dyn Cq, rhs: &dyn Cq) {
    //     if lhs.cq(rhs) {
    //         println!("cq ok.")
    //     } else {
    //         println!("cq failed.");
    //     }
    // }

    /// Compile error: ...because method `cq` references the `Self` type in this parameter
    // cq_tester(&Button, &CheckBox);

    // this is ok. for they are all Button type.
    assert!(&Button.cq(&Button));

    /// this function helps lhs and rhs has the same concrete type T: Cq
    fn cq_tester_ok<T: Cq>(lhs: &T, rhs: &T) {
        if lhs.cq(rhs) {
            println!("cq success");
        } else {
            eprintln!("cq failed here.");
        }
    }

    cq_tester_ok(&Button, &Button);

    /// this function compiles failed for lhs and rhs may have different concrete types.
    /// despite that, cq_tester_ok require the same concrete type of Cq for T
    // fn cq_tester_dyn(lhs: &dyn Cq, rhs: &dyn Cq) {
    //     cq_tester_ok(lhs, rhs);
    // }

    trait MQ: Sized {
        fn mq(&self) -> Self;
    }

    struct MQStruct {
        dest: String,
    }
    impl MQ for MQStruct {
        fn mq(&self) -> Self {
            MQStruct {
                dest: self.dest.clone(),
            }
        }
    }

    struct MQStruct2;
    impl MQ for MQStruct2 {
        fn mq(&self) -> Self {
            MQStruct2
        }
    }

    // compile error: because it requires `Self: Sized`
    // struct MQContainer {
    //     container: Vec<Box<dyn MQ>>,
    // }

    trait SomeTrait {
        fn foo(&self) -> u32 {
            0
        }

        // 把non-object-safe的方法拆分出来,放到单独的trait中,来保障object-safe.
        // fn new_self() -> Self;
    }
    trait SomeTraitExt: SomeTrait {
        fn new() -> Self;
    }

    fn some_trait_tester(lhs: &dyn SomeTrait, rhs: &dyn SomeTrait) {
        let _ = lhs.foo();
        let _ = rhs.foo();
    }

    struct Sxt;
    impl SomeTraitExt for Sxt {
        fn new() -> Self {
            Sxt
        }
    }

    struct St;
    impl SomeTrait for St {
        // fn new_self() -> Self {
        //     St
        // }
    }

    impl SomeTrait for Sxt {
        // fn new_self() -> Self {
        //     Sxt
        // }
    }

    some_trait_tester(&Sxt, &St);

    // Alternatives with where caluses for non-object-safe method.
    trait SomeTrait2 {
        // trait object not permit this definition.
        // const SIZE: usize = 1024;
        fn foo(&self) -> u32 {
            0
        }
        fn new() -> Self
        where
            Self: Sized;
    }

    // fn baz<T: ?Sized + SomeTrait2>(t: &T) {
    //     // illegal for T is Sized;
    //     let v: T = T::new();
    // }

    fn baz<T: SomeTrait2>(t: &T) {
        let v = T::new(); // its ok here.
    }

    trait SomeTrait3 {
        fn foo(&self) {
            println!("some trait3");
        }

        fn new() -> Self;
    }

    ///  compile error : doesn't have a size known at compile-time
    fn baz3<T: ?Sized + SomeTrait3>(t: &T) {
        // ^ doesn't have a size known at compile-time
        // let v = T::new();
    }

    /// not object-safe. this compile error.
    // fn baz4(x: &dyn SomeTrait3) {}
    ///this compiles ok for `new` function in SomeTrait2 returns a Self type with Sized restriction.
    /// which means a fix sized trait object can be created.
    fn baz5(x: &dyn SomeTrait2) {}

    /// error for : SomeTrait3 is not a object-safe trait.
    /// 不能使用&dyn SomeTrait3等语法
    // fn baz6(x: &Box<dyn SomeTrait3>) {}

    pub struct TraitObject {
        // ptr to the data object
        pub data: *mut (),
        // ptr to the trait vtables to search for functions to call.
        pub vtable: *mut (),
    }

    trait NonDispatchable {
        fn foo()
        where
            Self: Sized,
        {
        }
        fn returns(&self) -> Self
        where
            Self: Sized;
        fn param(&self, other: Self)
        where
            Self: Sized,
        {
        }
        fn typed<T>(&self, x: T)
        where
            Self: Sized,
        {
        }
    }

    struct S;
    impl NonDispatchable for S {
        fn returns(&self) -> Self
        where
            Self: Sized,
        {
            S
        }
    }

    let obj = S;
    obj.returns();
    obj.param(S);
    obj.typed(1);
    // these methods cannot be dispatched on a trait object.
    // dyn NonDispatchable.
    let boxed_s = Box::new(S) as Box<dyn NonDispatchable>;
    // has a Sized requirement.
    // boxed_s.returns();
    // boxed_s.param(S);
    // boxed_s.typed(1);

    let s = String::new();
    let s: &str = "hello";
}

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

struct Moveable {
    data: String,
    slice: NonNull<String>,
}

#[test]
fn test_pin() {
    let mut unmovable = Unmovable::new(String::from("hello world"));
    let s = unmovable;

    println!("s is : {:?}", s.slice);
    println!("data is : {:?}", NonNull::from(&s.data));

    let mut moveable = Moveable {
        data: String::from("hello world"),
        slice: NonNull::dangling(),
    };
    moveable.slice = NonNull::from(&moveable.data);
    let moved = moveable;
    println!("moved is : {:?}", moved.slice);
    println!("moved data is : {:?}", NonNull::from(&moved.data));
}
