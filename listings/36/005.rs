async fn append_after_wait(buffer: &mut String) {
    let view = buffer.as_str();
    log(view).await;
    buffer.push('!');
}
