use course_solutions::memory::c21::Node;
use std::rc::Rc;

#[test]
fn c21_tree_rejects_cycles_and_multiple_parents_without_changes() {
    let root = Node::new("root");
    let child = Node::new("child");
    let other = Node::new("other");
    Node::add_child(&root, Rc::clone(&child)).unwrap();
    assert!(Node::add_child(&root, Rc::clone(&root)).is_err());
    assert!(Node::add_child(&child, Rc::clone(&root)).is_err());
    assert!(Node::add_child(&other, Rc::clone(&child)).is_err());
    assert!(Node::add_child(&root, Rc::clone(&child)).is_err());
    assert_eq!(root.child_count(), 1);
    assert_eq!(child.child_count(), 0);
    assert_eq!(other.child_count(), 0);
    assert!(Rc::ptr_eq(&child.parent().unwrap(), &root));
}

#[test]
fn c21_child_outlives_parent_when_another_owner_remains() {
    let root = Node::new("root");
    let child = Node::new("child");
    Node::add_child(&root, Rc::clone(&child)).unwrap();
    let observed_root = Rc::downgrade(&root);
    drop(root);
    assert!(observed_root.upgrade().is_none());
    assert!(child.parent().is_none());
    assert_eq!(child.name, "child");
}

#[test]
fn c21_make_mut_disconnects_weak_references_without_cloning_the_value() {
    #[derive(Debug)]
    struct NoClone(u32);
    impl Clone for NoClone {
        fn clone(&self) -> Self {
            panic!("the unique value must not be cloned");
        }
    }
    let mut value = Rc::new(NoClone(1));
    let weak = Rc::downgrade(&value);
    assert!(Rc::get_mut(&mut value).is_none());
    Rc::make_mut(&mut value).0 = 2;
    assert!(weak.upgrade().is_none());
    assert_eq!(Rc::try_unwrap(value).unwrap().0, 2);
}
