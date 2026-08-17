attribute.parse_nested_meta(|meta| {
    if meta.path.is_ident("id") {
        options.id = Some(meta.value()?.parse()?);
        Ok(())
    } else if meta.path.is_ident("crate_path") {
        options.crate_path = Some(meta.value()?.parse()?);
        Ok(())
    } else {
        Err(meta.error("se esperaba `id` o `crate_path`"))
    }
})?;
