use course_solutions::organization::c29::{CI_COMMANDS, Port, PortError, Snapshot};

mod book_builder {
    include!("../../listings/29/006.rs");

    #[test]
    fn c29_the_builder_example_retains_the_returned_configuration() {
        main();
    }
}

#[test]
fn c29_a_consuming_conversion_reuses_the_existing_buffer() {
    let original = vec![1, 2, 3];
    let original_buffer = original.as_ptr();
    let snapshot = Snapshot::new(original);
    assert_eq!(snapshot.as_bytes().as_ptr(), original_buffer);
    let copy = snapshot.to_vec();
    assert_eq!(copy, snapshot.as_bytes());
    assert_ne!(copy.as_ptr(), original_buffer);
    let consumed = snapshot.into_bytes();
    assert_eq!(consumed.as_ptr(), original_buffer);
}

#[test]
fn c29_the_port_parser_reports_range_errors_as_documented() {
    assert_eq!(Port::parse("65535").unwrap().get(), u16::MAX);
    assert_eq!(Port::parse("65536"), Err(PortError::Invalid));
    assert_eq!(Port::parse("-1"), Err(PortError::Invalid));
    assert_eq!(Port::parse("0"), Err(PortError::Zero));
}

#[test]
fn c29_ci_includes_documentation_generation_for_all_workspace_members() {
    assert!(CI_COMMANDS.contains(&"cargo doc --workspace --no-deps --all-features --locked"));
    assert!(
        CI_COMMANDS
            .iter()
            .filter(|command| !command.contains(" fmt "))
            .all(|command| command.contains("--workspace"))
    );
}
