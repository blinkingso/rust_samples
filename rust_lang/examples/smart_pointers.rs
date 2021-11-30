use crate::List::{Cons, Nil};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// smart pointers
fn main() {}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn sp_box() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#[test]
fn test_deref() {
    let s = String::from("hello");
    let my_box = MyBox::new(s);
    assert_eq!("hello", *my_box);
    assert_eq!("hello", *my_box.deref());
    assert_eq!("hello", my_box.deref());
    let ref_my_box = my_box.deref();

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    assert_eq!(x, *y);

    assert_eq!(*"hello", *String::from("hello"));
}

#[derive(Debug)]
struct DropStr(String);
impl Drop for DropStr {
    fn drop(&mut self) {
        println!("str has been dropped");
    }
}

#[test]
fn test_drop() {
    let mut drop_str = DropStr(String::from("hello"));
    std::mem::drop(drop_str);
    // println!("{:?}", drop_str);
}

/// Rc<T> clone() new()
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[test]
fn test_rc() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("a's rc count: {}", Rc::strong_count(&a));
    let b = Rc::new(RcList::Cons(8, Rc::clone(&a)));
    println!("a's rc count: {}", Rc::strong_count(&a));
    {
        let d = Rc::new(RcList::Cons(-10, Rc::clone(&a)));
        println!("a's rc count: {}", Rc::strong_count(&a));
    }
    let c = Rc::new(RcList::Cons(12, Rc::clone(&a)));
    println!("a's rc count: {}", Rc::strong_count(&a));
}

/// RefCell<T> Runtime compile checker. borrow_mut(),
pub trait MessageSender {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: MessageSender> {
    sender: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: MessageSender,
{
    pub fn new(sender: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            sender,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent = self.value as f64 / self.max as f64;
        if percent >= 1.0 {
            println!("you are out of quota!");
        } else if percent >= 0.9 {
            println!("you are up to used of 90% quota!");
        } else if percent >= 0.75 {
            println!("you are now used 75% of quota!");
        }
    }
}

#[test]
fn test_ref_cell() {
    struct MockMessage {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessage {
        fn new() -> Self {
            MockMessage {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl MessageSender for MockMessage {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(String::from(message));
        }
    }

    let mock = MockMessage::new();
    mock.send("mocking message sending....");
    let mut tracker = LimitTracker::new(&mock, 100);
    tracker.set_value(80);
    assert_eq!(mock.messages.borrow().len(), 1);
    assert_eq!(mock.messages.borrow().len(), 1);
    mock.messages.borrow_mut().push(String::from("hello"));
    assert_eq!(mock.messages.borrow().len(), 2);
    tracker.set_value(101);
    mock.send("three");
    assert_eq!(mock.messages.borrow().len(), 3);
}

#[derive(Debug)]
enum LeakList {
    Cons(i32, RefCell<Rc<LeakList>>),
    Nil,
}

impl LeakList {
    pub fn tail(&self) -> Option<&RefCell<Rc<LeakList>>> {
        match self {
            LeakList::Cons(_, cons) => Some(cons),
            LeakList::Nil => None,
        }
    }
}

#[test]
fn test_leak_list() {
    let a = Rc::new(LeakList::Cons(
        32,
        RefCell::new(Rc::new(LeakList::Cons(
            10,
            RefCell::new(Rc::new(LeakList::Nil)),
        ))),
    ));

    println!("a initial rc count is {}", Rc::strong_count(&a));
    println!("a next item is : {:?}", a.tail());

    let b = Rc::new(LeakList::Cons(33, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation= {}", Rc::strong_count(&a));
    println!("b initial rc count is {}", Rc::strong_count(&b));
    println!("b next item is : {:?}", b.tail());

    if let Some(tail) = a.tail() {
        // code:: cycle ref
        *tail.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // overflow code for cycling reference.
    // println!("a next item = {:?}", a.tail());
    println!("a is now : {:?}", a);
}

#[test]
fn test_ref_cell2222() {
    let string = String::from("hello");
    let re = RefCell::new(string);
    re.borrow_mut().push_str(" world");
    assert_eq!("hello world", *re.borrow().deref());
}

/// LinkedList Node
pub struct Node<E> {
    value: E,
    prev: LinkedNode<E>,
    next: LinkedNode<E>,
}

impl<E> Node<E> {
    pub fn new(e: E) -> Node<E> {
        Node {
            value: e,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        }
    }

    pub fn next(&self, next: &Rc<Node<E>>) {
        *self.next.borrow_mut() = Some(Rc::clone(next));
    }
}

/// linked list new node
type LinkedNode<E> = RefCell<Option<Rc<Node<E>>>>;

pub struct Iter<E> {
    head: LinkedNode<E>,
    tail: LinkedNode<E>,
    size: usize,
    index: usize,
    next: LinkedNode<E>,
}

pub struct LinkedList<E> {
    head: LinkedNode<E>,
    tail: LinkedNode<E>,
    size: usize,
}

impl<E> LinkedList<E> {
    pub fn new() -> LinkedList<E> {
        LinkedList {
            head: RefCell::new(None),
            tail: RefCell::new(None),
            size: 0usize,
        }
    }

    pub fn iter(&self) -> Iter<E> {
        Iter {
            head: RefCell::clone(&self.head),
            tail: RefCell::clone(&self.tail),
            size: self.size,
            index: 0usize,
            next: RefCell::clone(&self.head),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add(&mut self, e: E) {
        let new_node = Rc::new(Node::new(e));
        if self.size == 0 {
            // first add.
            self.head = RefCell::new(Some(Rc::clone(&new_node)));
        } else {
            *self.tail.borrow_mut().as_ref().unwrap().next.borrow_mut() =
                Some(Rc::clone(&new_node));
        }
        self.tail = RefCell::new(Some(Rc::clone(&new_node)));
        // size ++
        self.size += 1;
    }

    /// pop node from the front node.
    #[inline]
    pub fn pop_front_node(&mut self) -> Option<Rc<Node<E>>> {
        // cp head ptr.
        return match self.head.clone().borrow().as_ref() {
            None => None,
            Some(head) => {
                // remove the front node
                let node = unsafe {
                    // get ownership
                    let node = Rc::from_raw(Rc::as_ptr(head));
                    // reset head.
                    self.head = RefCell::clone(&head.next.clone());
                    match self.head.borrow().as_ref() {
                        None => {
                            self.tail = RefCell::new(None);
                        }
                        // todo prev.
                        Some(_) => {}
                    }
                    node
                };

                self.size -= 1;
                Some(node)
            }
        };
    }

    #[doc = "ownership"]
    pub fn into_iter(self) -> IntoIter<E> {
        IntoIter { list: self }
    }
}

pub struct IntoIter<E> {
    list: LinkedList<E>,
}

impl<E> Iterator for Iter<E> {
    // linked list item.
    type Item = LinkedNode<E>;

    fn next(&mut self) -> Option<Self::Item> {
        // no elements or iterator has been finished.
        if self.size == 0 || self.index >= self.size {
            return None;
        }

        // first element
        let next = self.next.clone();
        let next_ = match next.borrow().as_ref().unwrap().next.borrow().as_ref() {
            None => None,
            Some(ne) => Some(ne.clone()),
        };
        // set new next
        *self.next.borrow_mut() = next_;
        self.index += 1;
        Some(next)
    }
}

#[test]
fn test_linked_list() {
    let mut list = LinkedList::new();
    list.add(1024);
    list.add(2048);
    list.add(4096);

    println!("to iterator...");
    for node in list.iter() {
        println!("v = {}", node.borrow().as_ref().unwrap().clone().value);
    }

    let head = list.pop_front_node();
    println!("head is : {}", head.unwrap().value);

    println!("======================");
    for node in list.iter() {
        println!("v = {}", node.borrow().as_ref().unwrap().clone().value);
    }
}
