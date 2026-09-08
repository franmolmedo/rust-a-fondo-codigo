use course_solutions::organization::c51::{
    DependencyRelease, FeatureProfile, PackageVersion, RustVersion, audit_dependency_releases,
    migration_matrix,
};

#[test]
fn c51_missing_metadata_is_not_treated_as_declared_compatibility() {
    let msrv = RustVersion::new(1, 86, 0);
    let audit = audit_dependency_releases(
        msrv,
        &[
            DependencyRelease {
                version: PackageVersion::new(1, 0, 0),
                rust_version: Some(msrv),
            },
            DependencyRelease {
                version: PackageVersion::new(1, 1, 0),
                rust_version: None,
            },
        ],
    );
    assert_eq!(
        audit.newest_declared_compatible,
        Some(PackageVersion::new(1, 0, 0))
    );
    assert_eq!(audit.undeclared_rust_versions, 1);
    assert_eq!(
        audit_dependency_releases(msrv, &[]).newest_declared_compatible,
        None
    );
}

#[test]
fn c51_matrix_deduplicates_every_dimension_and_empty_input_means_no_jobs() {
    let msrv = RustVersion::new(1, 86, 0);
    assert_eq!(
        migration_matrix(&[msrv, msrv], &[FeatureProfile::Default; 2], &["target"; 2]).len(),
        1
    );
    assert!(migration_matrix(&[], &[FeatureProfile::Default], &["target"]).is_empty());
    assert!(migration_matrix(&[msrv], &[], &["target"]).is_empty());
    assert!(migration_matrix(&[msrv], &[FeatureProfile::Default], &[]).is_empty());
}
