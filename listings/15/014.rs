trait Job {
    fn name(&self) -> &str;

    fn finish(self) -> String
    where
        Self: Sized,
    {
        format!("finalizado: {}", self.name())
    }
}

struct ImportJob(String);

impl Job for ImportJob {
    fn name(&self) -> &str {
        &self.0
    }
}

fn inspect(job: &dyn Job) -> &str {
    job.name()
}

fn main() {
    let job = ImportJob(String::from("usuarios"));
    assert_eq!(inspect(&job), "usuarios");
    assert_eq!(job.finish(), "finalizado: usuarios");
}
