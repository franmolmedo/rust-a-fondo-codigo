trait Job { fn run(&self); }

fn queue(job: impl Job + 'static) -> Box<dyn Job> {
    Box::new(job)
}
