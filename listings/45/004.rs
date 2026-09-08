fn second<T>(owner: &[T]) -> Option<&T> {
    let pointer = owner.as_ptr();
    (owner.len() > 1).then(|| {
        // SAFETY: the index is inside `owner`, and the returned reference
        // inherits exactly its shared lifetime.
        unsafe { &*pointer.add(1) }
    })
}

assert_eq!(second(&[10, 20, 30]), Some(&20));
