use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct EventLog {
    events: Rc<RefCell<Vec<String>>>,
}

impl EventLog {
    fn new() -> Self {
        Self { events: Rc::new(RefCell::new(Vec::new())) }
    }

    fn record(&self, event: impl Into<String>) {
        self.events.borrow_mut().push(event.into());
    }

    fn snapshot(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}
