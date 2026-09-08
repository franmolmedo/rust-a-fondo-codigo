//! External consumer that renames the dependency to test real paths and expansion.

pub struct Opaque;

#[derive(domain_api::Entity)]
#[entity(crate_path = domain_api, id = "id")]
pub struct GenericEntity<'a, T, const N: usize>
where
    T: 'a,
{
    pub id: u64,
    pub values: &'a [T; N],
}

pub const ENTITY_FIELDS: &[&str] = domain_api::field_names!(id, values);

#[domain_api::preserve_item]
#[inline]
pub fn answer() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_preserves_lifetime_type_const_and_where_clause() {
        let values = [Opaque];
        let entity = GenericEntity {
            id: 7,
            values: &values,
        };

        assert_eq!(entity.id, 7);
        assert_eq!(
            <GenericEntity<'_, Opaque, 1> as domain_api::Entity>::entity_name(),
            "GenericEntity",
        );
        assert_eq!(
            <GenericEntity<'_, Opaque, 1> as domain_api::Entity>::id_field(),
            "id",
        );
    }

    #[test]
    fn all_three_macro_families_execute_from_the_consumer() {
        assert_eq!(ENTITY_FIELDS, ["id", "values"]);
        assert_eq!(answer(), 42);
    }

    #[test]
    fn c50_generated_names_do_not_use_a_consumer_stringify_macro() {
        macro_rules! stringify {
            ($($tokens:tt)*) => {
                "shadowed"
            };
        }
        assert_eq!(stringify!(id), "shadowed");
        #[derive(domain_api::Entity)]
        #[entity(crate_path = domain_api, id = "id")]
        struct LocalEntity {
            id: u64,
        }
        assert_eq!(
            <LocalEntity as domain_api::Entity>::entity_name(),
            "LocalEntity"
        );
        assert_eq!(LocalEntity { id: 1 }.id, 1);
        let empty: &[&str] = domain_api::field_names!();
        assert!(empty.is_empty());
        assert_eq!(domain_api::field_names!(r#type,), &["r#type"]);
    }

    #[test]
    fn c50_attribute_preserves_const_generics_and_borrowed_output() {
        #[domain_api::preserve_item]
        const fn first<T, const N: usize>(values: &[T; N]) -> &T {
            &values[0]
        }
        const VALUE: &u32 = first(&[42]);
        assert_eq!(*VALUE, 42);
        let owner = [String::from("borrowed")];
        assert!(std::ptr::eq(first(&owner), &owner[0]));
    }
}
