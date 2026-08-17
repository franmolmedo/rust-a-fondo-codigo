trait Repository {
    fn find_name(&self, id: u64) -> Option<String>;
}

struct MemoryRepository;

impl Repository for MemoryRepository {
    fn find_name(&self, id: u64) -> Option<String> {
        (id == 1).then(|| String::from("Ada"))
    }
}

struct StaticService<R> {
    repository: R,
}

impl<R: Repository> StaticService<R> {
    fn name(&self, id: u64) -> Option<String> {
        self.repository.find_name(id)
    }
}

struct DynamicService {
    repository: Box<dyn Repository>,
}

impl DynamicService {
    fn name(&self, id: u64) -> Option<String> {
        self.repository.find_name(id)
    }
}

fn main() {
    let static_service = StaticService { repository: MemoryRepository };
    let dynamic_service = DynamicService { repository: Box::new(MemoryRepository) };
    assert_eq!(static_service.name(1).as_deref(), Some("Ada"));
    assert_eq!(dynamic_service.name(1).as_deref(), Some("Ada"));
}
