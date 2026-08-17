fn second<T>(owner: &[T]) -> Option<&T> {
    let pointer = owner.as_ptr();
    (owner.len() > 1).then(|| {
        // SAFETY: el índice está dentro de `owner` y la referencia devuelta
        // hereda exactamente su lifetime compartido.
        unsafe { &*pointer.add(1) }
    })
}

assert_eq!(second(&[10, 20, 30]), Some(&20));
