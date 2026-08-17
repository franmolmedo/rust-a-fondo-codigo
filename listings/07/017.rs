#[derive(Default)]
struct Measurements {
    values: Vec<i32>,
}

impl Measurements {
    fn add(&mut self, value: i32) {
        self.values.push(value);
    }

    fn remove_last(&mut self) -> Option<i32> {
        self.values.pop()
    }

    fn average(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }

        let total: i64 = self.values.iter().map(|&value| i64::from(value)).sum();
        Some(total as f64 / self.values.len() as f64)
    }
}

fn main() {
    let mut values = Measurements::default();
    assert_eq!(values.average(), None);

    values.add(10);
    values.add(20);
    assert_eq!(values.average(), Some(15.0));

    values.remove_last();
    assert_eq!(values.average(), Some(10.0));
}
