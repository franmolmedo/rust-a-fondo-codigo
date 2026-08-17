struct Command<'a> {
    verb: Verb,
    arguments: Vec<&'a str>,
}
