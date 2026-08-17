//! Consumidor externo: renombra la dependencia para probar paths y expansión reales.

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
}
