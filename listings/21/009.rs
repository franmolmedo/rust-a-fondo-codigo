let weak = Rc::downgrade(&config);
drop(config);
drop(another_owner);
assert!(weak.upgrade().is_none());
