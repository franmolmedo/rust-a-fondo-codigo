let values = [10, 20, 30, 40];

thread::scope(|scope| {
    let left = scope.spawn(|| values[..2].iter().sum::<i32>());
    let right = scope.spawn(|| values[2..].iter().sum::<i32>());

    assert_eq!(left.join().unwrap() + right.join().unwrap(), 100);
});
