fn once<F>(operation: F)
where
    F: FnOnce(),
{
    operation();
}

fn many<F>(mut operation: F)
where
    F: FnMut(),
{
    operation();
    operation();
}

fn shared<F>(operation: &F)
where
    F: Fn(),
{
    operation();
}

fn main() {
    once(|| println!("una"));

    let mut calls = 0;
    many(|| calls += 1);
    assert_eq!(calls, 2);

    let label = String::from("compartida");
    let show = || println!("{label}");
    shared(&show);
    shared(&show);
}
