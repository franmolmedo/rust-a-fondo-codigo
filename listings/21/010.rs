use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

let root = Rc::new(Node {
    name: String::from("root"),
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(Vec::new()),
});

let leaf = Rc::new(Node {
    name: String::from("leaf"),
    parent: RefCell::new(Rc::downgrade(&root)),
    children: RefCell::new(Vec::new()),
});

root.children.borrow_mut().push(Rc::clone(&leaf));

let parent_name = leaf.parent.borrow().upgrade().map(|node| node.name.clone());
assert_eq!(parent_name.as_deref(), Some("root"));
