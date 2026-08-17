let mut values = vec![1, 2, 3];
let first = &values[0];
values.push(4);
// error[E0502]: cannot borrow `values` as mutable
// because it is also borrowed as immutable
println!("{first}");
