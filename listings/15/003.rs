#[derive(Debug, PartialEq, Eq)]
struct Batch {
    values: Vec<u32>,
}

trait BatchOps {
    fn len(&self) -> usize;
    fn push(&mut self, value: u32);
    fn finish(self) -> Vec<u32>;
}

impl BatchOps for Batch {
    fn len(&self) -> usize { self.values.len() }
    fn push(&mut self, value: u32) { self.values.push(value); }
    fn finish(self) -> Vec<u32> { self.values }
}

fn main() {
    let mut batch = Batch { values: vec![1] };
    assert_eq!(batch.len(), 1);
    batch.push(2);
    assert_eq!(batch.finish(), [1, 2]);
}
