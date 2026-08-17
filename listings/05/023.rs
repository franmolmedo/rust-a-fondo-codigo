fn push_length(values: &mut Vec<usize>, length: usize) {
    values.push(length);
}

fn main() {
    let mut values = vec![10, 20, 30];
    push_length(&mut values, values.len());
    // error[E0502]: explicit mutable and shared borrows overlap
}
