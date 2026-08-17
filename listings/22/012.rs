use std::cell::RefCell;

struct Dispatcher {
    events: RefCell<Vec<&'static str>>,
}

impl Dispatcher {
    fn emit(&self, callback: impl FnOnce(&Self)) {
        {
            let mut events = self.events.borrow_mut();
            events.push("emit");
        } // RefMut destruido antes de la callback

        callback(self);
    }

    fn count(&self) -> usize {
        self.events.borrow().len()
    }
}

let dispatcher = Dispatcher { events: RefCell::new(Vec::new()) };
dispatcher.emit(|same| assert_eq!(same.count(), 1));
