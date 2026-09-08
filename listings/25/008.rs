pub mod domain {
    pub struct Api;
    pub(crate) struct CrateInternal;

    mod validation {
        pub(super) fn parent_visible() {}
        pub(in crate::domain) fn domain_visible() {}
    }
}
