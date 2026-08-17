let future = async {
    let left = load_left().await?;
    let right = load_right().await?;
    Ok::<_, LoadError>((left, right))
};
