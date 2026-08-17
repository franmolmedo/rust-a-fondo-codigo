trait Task {
    fn name(&self) -> &str;
}

struct Job(&'static str);

impl Task for Job {
    fn name(&self) -> &str {
        self.0
    }
}

fn assert_thread_safe<T: Send + Sync>(_value: &T) {}

fn main() {
    let task: Box<dyn Task + Send + Sync> = Box::new(Job("backup"));
    assert_thread_safe(&task);
    assert_eq!(task.name(), "backup");
}
