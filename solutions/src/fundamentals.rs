//! Capítulos 1 a 10: modelo mental, expresiones, ownership, préstamos, tipos y errores.

pub mod c01 {
    // SOLUTION: C01-E05
    pub fn option_pointer_sizes() -> ((usize, usize), (usize, usize)) {
        (
            (size_of::<Option<&u8>>(), size_of::<&u8>()),
            (size_of::<Option<Box<u64>>>(), size_of::<Box<u64>>()),
        )
    }

    // SOLUTION: C01-E06
    pub fn greet(name: &str) -> String {
        format!("hola, {name}")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn documented_pointer_niches_add_no_size() {
            let (borrowed, owned) = option_pointer_sizes();
            assert_eq!(borrowed.0, borrowed.1);
            assert_eq!(owned.0, owned.1);
        }

        #[test]
        fn greeting_borrows_instead_of_consuming() {
            let name = String::from("Ada");
            assert_eq!(greet(&name), "hola, Ada");
            assert_eq!(name, "Ada");
        }
    }
}

pub mod c02 {
    // SOLUTION: C02-E01
    #[allow(clippy::let_and_return)] // conserva exactamente la forma que pide predecir el ejercicio
    pub fn block_value() -> i32 {
        let x = {
            let a = 10;
            a * 2
        };
        x
    }

    // SOLUTION: C02-E03
    #[allow(clippy::let_and_return)] // el último binding forma parte de la demostración de shadowing
    pub fn shadowed_value() -> i32 {
        let x = 5;
        let x = x + 1;
        let x = x * 10;
        x
    }

    // SOLUTION: C02-E05
    pub fn label_for(score: u8) -> &'static str {
        if score >= 50 { "aprobado" } else { "suspenso" }
    }

    // SOLUTION: C02-E06
    #[allow(clippy::manual_find)] // el ejercicio exige practicar `loop` y `break value`
    pub fn first_even(values: &[i32]) -> Option<i32> {
        let mut index = 0;

        loop {
            if index == values.len() {
                break None;
            }

            let value = values[index];
            if value % 2 == 0 {
                break Some(value);
            }

            index += 1;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn expressions_return_the_predicted_values() {
            assert_eq!(block_value(), 20);
            assert_eq!(shadowed_value(), 60);
        }

        #[test]
        fn conditional_expression_eliminates_intermediate_state() {
            assert_eq!(label_for(49), "suspenso");
            assert_eq!(label_for(50), "aprobado");
        }

        #[test]
        fn loop_returns_the_first_even_value() {
            assert_eq!(first_even(&[]), None);
            assert_eq!(first_even(&[1, 7]), None);
            assert_eq!(first_even(&[1, 7, 4, 8]), Some(4));
        }
    }
}

pub mod c03 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountId(pub u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SourceAccountId(pub AccountId);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct DestinationAccountId(pub AccountId);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Money {
        pub cents: u64,
        pub currency: &'static str,
    }

    // SOLUTION: C03-E02
    pub fn count_words(input: &str) -> usize {
        input.split_whitespace().count()
    }

    // SOLUTION: C03-E03
    pub fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }

    // SOLUTION: C03-E05
    pub fn transfer_description(
        source: SourceAccountId,
        destination: DestinationAccountId,
        money: Money,
    ) -> String {
        format!(
            "{} -> {}: {} {}",
            source.0.0, destination.0.0, money.cents, money.currency
        )
    }

    // SOLUTION: C03-E07
    pub fn add_stock(current: u32, incoming: u32) -> Option<u32> {
        current.checked_add(incoming)
    }

    // SOLUTION: C03-E08
    pub fn inspect_first(input: &str) -> (Option<char>, Option<u8>) {
        (input.chars().next(), input.as_bytes().first().copied())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn borrowed_inputs_are_not_consumed() {
            let text = String::from("Rust hace explícito el contrato");
            assert_eq!(count_words(&text), 5);
            assert_eq!(text.len(), 32);
            assert_eq!(sum(&[2, 3, 5]), 10);
        }

        #[test]
        fn newtypes_keep_roles_distinct() {
            let description = transfer_description(
                SourceAccountId(AccountId(1)),
                DestinationAccountId(AccountId(2)),
                Money {
                    cents: 500,
                    currency: "EUR",
                },
            );
            assert_eq!(description, "1 -> 2: 500 EUR");
        }

        #[test]
        fn stock_overflow_is_explicit() {
            assert_eq!(add_stock(40, 2), Some(42));
            assert_eq!(add_stock(u32::MAX, 1), None);
        }

        #[test]
        fn unicode_scalar_and_first_byte_are_distinct() {
            assert_eq!(inspect_first("é"), (Some('é'), Some(0xc3)));
            assert_eq!(inspect_first(""), (None, None));
        }
    }
}

pub mod c04 {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Order {
        items: Vec<u64>,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct SubmittedOrder {
        items: Vec<u64>,
        total_cents: u64,
    }

    impl Order {
        pub fn new() -> Self {
            Self { items: Vec::new() }
        }

        // SOLUTION: C04-E04
        pub fn total(&self) -> u64 {
            self.items.iter().sum()
        }

        pub fn add_item(&mut self, cents: u64) {
            self.items.push(cents);
        }

        pub fn submit(self) -> SubmittedOrder {
            let total_cents = self.total();
            SubmittedOrder {
                items: self.items,
                total_cents,
            }
        }
    }

    impl Default for Order {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SubmittedOrder {
        pub fn total(&self) -> u64 {
            self.total_cents
        }

        pub fn item_count(&self) -> usize {
            self.items.len()
        }
    }

    #[derive(Debug, Default)]
    pub struct UserBuilder {
        name: Option<String>,
        email: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct User {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum BuildError {
        MissingName,
        MissingEmail,
    }

    // SOLUTION: C04-E05
    impl UserBuilder {
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        pub fn email(mut self, email: impl Into<String>) -> Self {
            self.email = Some(email.into());
            self
        }

        pub fn build(self) -> Result<User, BuildError> {
            Ok(User {
                name: self.name.ok_or(BuildError::MissingName)?,
                email: self.email.ok_or(BuildError::MissingEmail)?,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn receivers_express_observe_mutate_and_consume() {
            let mut order = Order::new();
            order.add_item(200);
            order.add_item(300);
            assert_eq!(order.total(), 500);
            let submitted = order.submit();
            assert_eq!(submitted.total(), 500);
            assert_eq!(submitted.item_count(), 2);
        }

        #[test]
        fn builder_validates_at_the_final_boundary() {
            let user = UserBuilder::default()
                .name("Ada")
                .email("ada@example.test")
                .build()
                .unwrap();
            assert_eq!(user.name, "Ada");
            assert_eq!(
                UserBuilder::default().name("Ada").build(),
                Err(BuildError::MissingEmail)
            );
        }
    }
}

pub mod c05 {
    // SOLUTION: C05-E01
    pub fn text_length(text: &str) -> usize {
        text.len()
    }

    // SOLUTION: C05-E02
    pub fn contains_name(names: &[String], wanted: &str) -> bool {
        names.iter().any(|name| name == wanted)
    }

    // SOLUTION: C05-E03
    pub fn append_after_last_use(mut text: String) -> (usize, String) {
        let view = &text;
        let length_before = view.len();
        text.push('!');
        (length_before, text)
    }

    pub fn append_after_scoped_read(mut text: String) -> (usize, String) {
        let length_before = {
            let view = &text;
            view.len()
        };
        text.push('!');
        (length_before, text)
    }

    // SOLUTION: C05-E04
    pub fn normalize_owned(mut text: String) -> String {
        let start = text.len() - text.trim_start().len();
        let end = text.trim_end().len();

        if start >= end {
            text.clear();
        } else {
            text.truncate(end);
            text.drain(..start);
            text.make_ascii_lowercase();
        }
        text
    }

    pub fn normalize_borrowed(text: &str) -> String {
        text.trim().to_ascii_lowercase()
    }

    // SOLUTION: C05-E05
    pub fn inspect_then_push(values: &mut Vec<String>, new_value: String) -> Option<usize> {
        let first_length = values.first().map(String::len);
        values.push(new_value);
        first_length
    }

    pub fn push_then_reborrow(values: &mut Vec<String>, new_value: String) -> Option<&str> {
        let first_index = (!values.is_empty()).then_some(0);
        values.push(new_value);
        first_index.map(|index| values[index].as_str())
    }

    // SOLUTION: C05-E06
    pub fn increment_ends(values: &mut [i32]) {
        match values.len() {
            0 => {}
            1 => values[0] += 1,
            length => {
                let (left, right) = values.split_at_mut(length - 1);
                left[0] += 1;
                right[0] += 1;
            }
        }
    }

    // SOLUTION: C05-E07
    pub fn increment_twice_nll() -> u32 {
        let mut count = 0;
        let mut increment = || count += 1;
        increment();
        increment();
        count
    }

    pub fn increment_twice_scoped() -> u32 {
        let mut count = 0;
        {
            let mut increment = || count += 1;
            increment();
            increment();
        }
        count
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn apis_accept_the_weakest_useful_borrow() {
            let text = String::from("borrowed");
            assert_eq!(text_length(&text), 8);
            assert_eq!(text_length("literal"), 7);
            assert_eq!(text, "borrowed");

            let names = vec![String::from("Ada"), String::from("Grace")];
            assert!(contains_name(&names, "Grace"));

            let names = [String::from("Linus"), String::from("Margaret")];
            assert!(contains_name(&names, "Linus"));
        }

        #[test]
        fn reading_and_writing_happen_in_separate_phases() {
            assert_eq!(
                append_after_last_use(String::from("hola")),
                (4, String::from("hola!"))
            );
            assert_eq!(
                append_after_scoped_read(String::from("Rust")),
                (4, String::from("Rust!"))
            );
        }

        #[test]
        fn owned_and_borrowed_normalization_have_distinct_contracts() {
            let original = String::from("  RuSt  ");
            let normalized = normalize_borrowed(&original);
            assert_eq!(original, "  RuSt  ");
            assert_eq!(normalized, "rust");

            assert_eq!(normalize_owned(String::from("  RuSt  ")), "rust");
            assert_eq!(normalize_owned(String::from("   ")), "");
        }

        #[test]
        fn vector_is_reborrowed_only_after_push() {
            let mut values = vec![String::from("uno")];
            assert_eq!(inspect_then_push(&mut values, String::from("dos")), Some(3));
            assert_eq!(
                push_then_reborrow(&mut values, String::from("tres")),
                Some("uno")
            );
        }

        #[test]
        fn split_at_mut_proves_that_endpoints_are_disjoint() {
            let mut empty: [i32; 0] = [];
            increment_ends(&mut empty);
            assert_eq!(empty, [0_i32; 0]);

            let mut one = [10];
            increment_ends(&mut one);
            assert_eq!(one, [11]);

            let mut many = [10, 20, 30];
            increment_ends(&mut many);
            assert_eq!(many, [11, 20, 31]);
        }

        #[test]
        fn closure_borrow_ends_after_its_last_use_or_scope() {
            assert_eq!(increment_twice_nll(), 2);
            assert_eq!(increment_twice_scoped(), 2);
        }
    }
}

pub mod c06 {
    use thiserror::Error;

    // SOLUTION: C06-E01
    pub fn make_owned() -> String {
        String::from("hola")
    }

    pub fn first_word(input: &str) -> &str {
        input.split_whitespace().next().unwrap_or("")
    }

    // SOLUTION: C06-E02
    pub fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() >= second.len() {
            first
        } else {
            second
        }
    }

    // SOLUTION: C06-E03
    #[allow(clippy::needless_lifetimes)] // hace visibles los dos lifetimes independientes del enunciado
    pub fn first<'a, 'b>(left: &'a str, _right: &'b str) -> &'a str {
        left
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        name: String,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct UserView<'a> {
        pub name: &'a str,
    }

    // SOLUTION: C06-E04
    impl User {
        pub fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }

        pub fn as_view(&self) -> UserView<'_> {
            UserView { name: &self.name }
        }

        pub fn into_name(self) -> String {
            self.name
        }
    }

    // SOLUTION: C06-E05
    pub fn length_then_clear(text: &mut String) -> usize {
        let length = first_word(text).len();
        text.clear();
        length
    }

    pub fn owned_word_then_clear(text: &mut String) -> String {
        let word = first_word(text).to_owned();
        text.clear();
        word
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Email(String);

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum EmailError {
        #[error("el email está vacío")]
        Empty,
        #[error("el email no contiene @")]
        MissingAt,
    }

    // SOLUTION: C06-E06
    impl Email {
        pub fn parse(value: String) -> Result<Self, EmailError> {
            if value.is_empty() {
                return Err(EmailError::Empty);
            }
            if !value.contains('@') {
                return Err(EmailError::MissingAt);
            }
            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }

        pub fn into_inner(self) -> String {
            self.0
        }
    }

    // SOLUTION: C06-E07
    pub fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
        input.lines().map(str::trim).filter(|line| !line.is_empty())
    }

    pub fn normalized_lines(input: &str) -> Vec<String> {
        non_empty_lines(input).map(str::to_lowercase).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn output_is_owned_or_borrowed_from_an_input() {
            assert_eq!(make_owned(), "hola");

            let input = String::from("Rust seguro");
            assert_eq!(first_word(&input), "Rust");
            assert_eq!(input, "Rust seguro");
        }

        #[test]
        fn longest_is_limited_to_the_common_region() {
            let outer = String::from("exterior");
            {
                let inner = String::from("interior más largo");
                assert_eq!(longest(&outer, &inner), "interior más largo");
            }
            assert_eq!(outer, "exterior");
        }

        #[test]
        fn result_lifetime_depends_only_on_the_first_input() {
            let long = String::from("largo");
            let result;
            {
                let short = String::from("corto");
                result = first(&long, &short);
            }
            assert_eq!(result, "largo");
        }

        #[test]
        fn owned_entity_can_produce_a_temporary_view() {
            let user = User::new("Ada");
            assert_eq!(user.as_view(), UserView { name: "Ada" });
            assert_eq!(user.into_name(), "Ada");
        }

        #[test]
        fn mutation_happens_after_the_view_is_finished_or_owned() {
            let mut first = String::from("hola mundo");
            assert_eq!(length_then_clear(&mut first), 4);
            assert!(first.is_empty());

            let mut second = String::from("Rust seguro");
            assert_eq!(owned_word_then_clear(&mut second), "Rust");
            assert!(second.is_empty());
        }

        #[test]
        fn owned_value_object_has_borrow_and_consume_views() {
            let email = Email::parse(String::from("ada@example.test")).unwrap();
            assert_eq!(email.as_str(), "ada@example.test");
            assert_eq!(email.into_inner(), "ada@example.test");
            assert_eq!(
                Email::parse(String::from("invalid")),
                Err(EmailError::MissingAt)
            );
        }

        #[test]
        fn borrowed_and_owned_iterators_have_distinct_lifecycles() {
            let input = String::from(" UNO \n\n DOS ");
            assert_eq!(non_empty_lines(&input).collect::<Vec<_>>(), ["UNO", "DOS"]);

            let owned = normalized_lines(&input);
            drop(input);
            assert_eq!(owned, ["uno", "dos"]);
        }
    }
}

pub mod c07 {
    use std::fmt;
    use std::num::NonZeroU32;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct UserId(u64);

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Email(String);

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Username(String);

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        id: UserId,
        email: Email,
        username: Username,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ValueError {
        Empty,
        MissingAt,
    }

    // SOLUTION: C07-E01
    impl UserId {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn get(self) -> u64 {
            self.0
        }
    }

    impl Email {
        pub fn parse(value: impl Into<String>) -> Result<Self, ValueError> {
            let value = value.into();
            if value.is_empty() {
                return Err(ValueError::Empty);
            }
            if !value.contains('@') {
                return Err(ValueError::MissingAt);
            }
            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl Username {
        pub fn parse(value: impl Into<String>) -> Result<Self, ValueError> {
            let value = value.into();
            if value.trim().is_empty() {
                Err(ValueError::Empty)
            } else {
                Ok(Self(value))
            }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl User {
        pub fn new(id: UserId, email: Email, username: Username) -> Self {
            Self {
                id,
                email,
                username,
            }
        }

        pub fn id(&self) -> UserId {
            self.id
        }

        pub fn username(&self) -> &Username {
            &self.username
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Quantity(NonZeroU32);

    // SOLUTION: C07-E02
    impl Quantity {
        pub fn new(value: u32) -> Option<Self> {
            NonZeroU32::new(value).map(Self)
        }

        pub fn get(self) -> u32 {
            self.0.get()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ProductId(u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CartItem {
        product: ProductId,
        quantity: u32,
    }

    #[derive(Default)]
    pub struct Cart {
        items: Vec<CartItem>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CartError {
        ZeroQuantity,
        QuantityOverflow,
    }

    // SOLUTION: C07-E03
    impl Cart {
        pub fn add(&mut self, product: ProductId, quantity: u32) -> Result<(), CartError> {
            if quantity == 0 {
                return Err(CartError::ZeroQuantity);
            }

            if let Some(item) = self.items.iter_mut().find(|item| item.product == product) {
                item.quantity = item
                    .quantity
                    .checked_add(quantity)
                    .ok_or(CartError::QuantityOverflow)?;
            } else {
                self.items.push(CartItem { product, quantity });
            }
            Ok(())
        }

        pub fn items(&self) -> &[CartItem] {
            &self.items
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct PasswordHash(String);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct EmptyPasswordHash;

    // SOLUTION: C07-E04
    impl PasswordHash {
        pub fn parse(value: String) -> Result<Self, EmptyPasswordHash> {
            (!value.is_empty())
                .then_some(Self(value))
                .ok_or(EmptyPasswordHash)
        }

        pub fn verify_for_test(&self, expected: &str) -> bool {
            self.0 == expected
        }
    }

    impl fmt::Debug for PasswordHash {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("PasswordHash([REDACTED])")
        }
    }

    // SOLUTION: C07-E05
    impl User {
        pub fn email(&self) -> &Email {
            &self.email
        }

        pub fn replace_email(&mut self, email: Email) -> Email {
            std::mem::replace(&mut self.email, email)
        }

        pub fn into_email(self) -> Email {
            self.email
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ServerConfig {
        host: String,
        port: u16,
        workers: usize,
    }

    #[derive(Default)]
    pub struct ServerConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        workers: Option<usize>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ConfigError {
        MissingHost,
        MissingPort,
        ZeroWorkers,
    }

    // SOLUTION: C07-E06
    impl ServerConfigBuilder {
        pub fn host(mut self, value: impl Into<String>) -> Self {
            self.host = Some(value.into());
            self
        }

        pub fn port(mut self, value: u16) -> Self {
            self.port = Some(value);
            self
        }

        pub fn workers(mut self, value: usize) -> Self {
            self.workers = Some(value);
            self
        }

        pub fn build(self) -> Result<ServerConfig, ConfigError> {
            let host = self.host.ok_or(ConfigError::MissingHost)?;
            let port = self.port.ok_or(ConfigError::MissingPort)?;
            let workers = self.workers.unwrap_or(4);
            if workers == 0 {
                return Err(ConfigError::ZeroWorkers);
            }
            Ok(ServerConfig {
                host,
                port,
                workers,
            })
        }
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PublishedPost {
        content: String,
    }

    // SOLUTION: C07-E07
    impl DraftPost {
        pub fn new() -> Self {
            Self {
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn publish(self) -> PublishedPost {
            PublishedPost {
                content: self.content,
            }
        }
    }

    impl Default for DraftPost {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PublishedPost {
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn user_is_built_from_validated_concepts() {
            let user = User::new(
                UserId::new(7),
                Email::parse("ada@example.test").unwrap(),
                Username::parse("ada").unwrap(),
            );
            assert_eq!(user.id().get(), 7);
            assert_eq!(user.email().as_str(), "ada@example.test");
            assert_eq!(user.username().as_str(), "ada");
            assert_eq!(Email::parse("invalid"), Err(ValueError::MissingAt));
        }

        #[test]
        fn quantity_cannot_represent_zero() {
            assert_eq!(Quantity::new(0), None);
            assert_eq!(Quantity::new(3).map(Quantity::get), Some(3));
        }

        #[test]
        fn cart_preserves_quantity_and_uniqueness_rules() {
            let mut cart = Cart::default();
            assert_eq!(cart.add(ProductId(1), 0), Err(CartError::ZeroQuantity));
            cart.add(ProductId(1), 2).unwrap();
            cart.add(ProductId(1), 3).unwrap();
            assert_eq!(
                cart.items(),
                [CartItem {
                    product: ProductId(1),
                    quantity: 5,
                }]
            );
        }

        #[test]
        fn debug_never_reveals_the_hash() {
            let hash = PasswordHash::parse(String::from("secret-hash")).unwrap();
            assert!(hash.verify_for_test("secret-hash"));
            assert_eq!(format!("{hash:?}"), "PasswordHash([REDACTED])");
        }

        #[test]
        fn receivers_express_observation_replacement_and_consumption() {
            let mut user = User::new(
                UserId::new(7),
                Email::parse("old@example.test").unwrap(),
                Username::parse("ada").unwrap(),
            );
            let old = user.replace_email(Email::parse("new@example.test").unwrap());
            assert_eq!(old.as_str(), "old@example.test");
            assert_eq!(user.into_email().as_str(), "new@example.test");
        }

        #[test]
        fn builder_validates_the_final_boundary() {
            let config = ServerConfigBuilder::default()
                .host("127.0.0.1")
                .port(8080)
                .build()
                .unwrap();
            assert_eq!(config.workers, 4);
            assert_eq!(
                ServerConfigBuilder::default().port(8080).build(),
                Err(ConfigError::MissingHost)
            );
        }

        #[test]
        fn publishing_consumes_the_draft() {
            let mut draft = DraftPost::new();
            draft.add_text("contenido");
            let published = draft.publish();
            assert_eq!(published.content(), "contenido");
        }
    }
}

pub mod c08 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderStatus {
        Draft,
        Submitted,
        Paid,
        Cancelled,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct UnknownOrderStatus;

    // SOLUTION: C08-E01
    impl TryFrom<&str> for OrderStatus {
        type Error = UnknownOrderStatus;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "draft" => Ok(Self::Draft),
                "submitted" => Ok(Self::Submitted),
                "paid" => Ok(Self::Paid),
                "cancelled" => Ok(Self::Cancelled),
                _ => Err(UnknownOrderStatus),
            }
        }
    }

    // SOLUTION: C08-E02
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Message {
        Quit,
        Text(String),
        Move { x: i32, y: i32 },
    }

    // SOLUTION: C08-E03
    pub fn can_cancel(status: OrderStatus) -> bool {
        matches!(status, OrderStatus::Draft | OrderStatus::Submitted)
    }

    impl OrderStatus {
        pub fn can_cancel(self) -> bool {
            matches!(self, Self::Draft | Self::Submitted)
        }
    }

    // SOLUTION: C08-E04
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum UsernameError {
        Empty,
        TooShort { minimum: usize, actual: usize },
        TooLong { maximum: usize, actual: usize },
        InvalidCharacter { index: usize, character: char },
    }

    pub fn validate_username(input: &str) -> Result<(), UsernameError> {
        const MINIMUM: usize = 3;
        const MAXIMUM: usize = 20;
        let length = input.chars().count();

        if length == 0 {
            return Err(UsernameError::Empty);
        }
        if length < MINIMUM {
            return Err(UsernameError::TooShort {
                minimum: MINIMUM,
                actual: length,
            });
        }
        if length > MAXIMUM {
            return Err(UsernameError::TooLong {
                maximum: MAXIMUM,
                actual: length,
            });
        }
        if let Some((index, character)) = input
            .char_indices()
            .find(|(_, character)| !character.is_ascii_alphanumeric() && *character != '_')
        {
            return Err(UsernameError::InvalidCharacter { index, character });
        }

        Ok(())
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub id: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum QueryError {
        NotFound { id: u64 },
        Unavailable,
    }

    // SOLUTION: C08-E05
    pub fn find_optional(users: &[User], id: u64) -> Option<&User> {
        users.iter().find(|user| user.id == id)
    }

    pub fn require_user(users: &[User], id: u64) -> Result<&User, QueryError> {
        find_optional(users, id).ok_or(QueryError::NotFound { id })
    }

    pub fn query_user(
        result: Result<Option<User>, QueryError>,
    ) -> Result<Option<User>, QueryError> {
        result
    }

    // SOLUTION: C08-E06
    pub fn text(message: &Message) -> Option<&str> {
        match message {
            Message::Text(text) => Some(text),
            Message::Quit | Message::Move { .. } => None,
        }
    }

    pub fn append_text(message: &mut Message, suffix: &str) -> bool {
        if let Message::Text(text) = message {
            text.push_str(suffix);
            true
        } else {
            false
        }
    }

    // SOLUTION: C08-E07
    pub fn first_non_empty_line(input: &str) -> Option<&str> {
        input.lines().map(str::trim).find(|line| !line.is_empty())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn external_status_is_validated_before_entering_the_domain() {
            assert_eq!(OrderStatus::try_from("paid"), Ok(OrderStatus::Paid));
            assert_eq!(OrderStatus::try_from("unknown"), Err(UnknownOrderStatus));
        }

        #[test]
        fn variants_and_rules_are_exhaustive() {
            let message = Message::Move { x: 3, y: 4 };
            assert!(matches!(message, Message::Move { x: 3, y: 4 }));
            assert!(can_cancel(OrderStatus::Draft));
            assert_eq!(
                can_cancel(OrderStatus::Submitted),
                OrderStatus::Submitted.can_cancel()
            );
            assert!(!OrderStatus::Paid.can_cancel());
        }

        #[test]
        fn errors_and_query_contracts_preserve_meaning() {
            assert_eq!(
                validate_username("a!"),
                Err(UsernameError::TooShort {
                    minimum: 3,
                    actual: 2,
                })
            );
            assert_eq!(
                validate_username("ada!"),
                Err(UsernameError::InvalidCharacter {
                    index: 3,
                    character: '!',
                })
            );

            let users = [User { id: 7 }];
            assert_eq!(find_optional(&users, 8), None);
            assert_eq!(require_user(&users, 7), Ok(&users[0]));
            assert_eq!(require_user(&users, 8), Err(QueryError::NotFound { id: 8 }));
            assert_eq!(query_user(Ok(None)), Ok(None));
            assert_eq!(
                query_user(Err(QueryError::Unavailable)),
                Err(QueryError::Unavailable)
            );
        }

        #[test]
        fn payloads_can_be_observed_and_mutated_without_consuming_the_enum() {
            let mut message = Message::Text(String::from("hola"));
            assert_eq!(text(&message), Some("hola"));
            assert!(append_text(&mut message, "!"));
            assert_eq!(text(&message), Some("hola!"));

            let input = String::from("\n  \n Rust \n");
            assert_eq!(first_non_empty_line(&input), Some("Rust"));
        }
    }
}

pub mod c09 {
    // SOLUTION: C09-E01
    #[allow(clippy::match_single_binding)] // El ejercicio demuestra que el patrón captura.
    pub fn captured_value(value: i32, outer: i32) -> (i32, i32) {
        let captured = match value {
            outer => outer,
        };
        (captured, outer)
    }

    pub fn equals_outer(value: i32, outer: i32) -> bool {
        matches!(value, candidate if candidate == outer)
    }

    pub const SPECIAL: i32 = 42;

    #[derive(Debug, Eq, PartialEq)]
    pub enum Message {
        Text(String),
        Quit,
    }

    // SOLUTION: C09-E02
    pub fn inspect_text(message: &Message) -> Option<&str> {
        match message {
            Message::Text(text) => Some(text),
            Message::Quit => None,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderStatus {
        Draft,
        Submitted,
        Paid,
        Cancelled,
    }

    // SOLUTION: C09-E03
    pub fn can_cancel_with_match(status: OrderStatus) -> bool {
        match status {
            OrderStatus::Draft | OrderStatus::Submitted => true,
            OrderStatus::Paid | OrderStatus::Cancelled => false,
        }
    }

    pub fn can_cancel_with_matches(status: OrderStatus) -> bool {
        matches!(status, OrderStatus::Draft | OrderStatus::Submitted)
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Payment {
        Cash,
        Card { last_four: String },
        Transfer { reference: String },
    }

    // SOLUTION: C09-E04
    pub fn describe_payment(payment: &Payment) -> String {
        match payment {
            Payment::Cash => String::from("efectivo"),
            Payment::Card { last_four } => format!("tarjeta terminada en {last_four}"),
            Payment::Transfer { reference } => format!("transferencia {reference}"),
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum ParsedArgs<'a> {
        Help {
            program: &'a str,
        },
        Command {
            program: &'a str,
            command: &'a str,
            rest: &'a [String],
        },
    }

    // SOLUTION: C09-E05
    pub fn parse_args(args: &[String]) -> Option<ParsedArgs<'_>> {
        match args {
            [program] => Some(ParsedArgs::Help { program }),
            [program, command, rest @ ..] => Some(ParsedArgs::Command {
                program,
                command,
                rest,
            }),
            [] => None,
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum ServiceError {
        Http { status: u16, path: String },
        Timeout,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Severity {
        Severe,
        Normal,
    }

    // SOLUTION: C09-E06
    pub fn severity(error: &ServiceError) -> Severity {
        match error {
            ServiceError::Http { status, .. } if *status >= 500 => Severity::Severe,
            ServiceError::Http { .. } | ServiceError::Timeout => Severity::Normal,
        }
    }

    // SOLUTION: C09-E07
    pub fn require_user(user: Option<&str>) -> Result<&str, &'static str> {
        let Some(user) = user else {
            return Err("usuario no encontrado");
        };
        Ok(user)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn bindings_capture_but_guards_and_constants_compare() {
            assert_eq!(captured_value(20, 10), (20, 10));
            assert!(equals_outer(10, 10));
            assert!(!equals_outer(20, 10));
            assert!(matches!(42, SPECIAL));
        }

        #[test]
        fn borrowed_matches_preserve_owned_payloads() {
            let message = Message::Text(String::from("hola"));
            assert_eq!(inspect_text(&message), Some("hola"));
            assert_eq!(message, Message::Text(String::from("hola")));

            let payment = Payment::Card {
                last_four: String::from("4242"),
            };
            assert_eq!(describe_payment(&payment), "tarjeta terminada en 4242");
            assert!(matches!(payment, Payment::Card { .. }));
        }

        #[test]
        fn exhaustive_predicates_and_guards_cover_fallbacks() {
            for status in [
                OrderStatus::Draft,
                OrderStatus::Submitted,
                OrderStatus::Paid,
                OrderStatus::Cancelled,
            ] {
                assert_eq!(
                    can_cancel_with_match(status),
                    can_cancel_with_matches(status)
                );
            }

            assert_eq!(
                severity(&ServiceError::Http {
                    status: 503,
                    path: String::from("/users"),
                }),
                Severity::Severe
            );
            assert_eq!(
                severity(&ServiceError::Http {
                    status: 404,
                    path: String::from("/users"),
                }),
                Severity::Normal
            );
        }

        #[test]
        fn slice_patterns_express_arity_and_remainder() {
            let args = vec!["app".into(), "serve".into(), "--port".into(), "8080".into()];
            let parsed = parse_args(&args).unwrap();
            assert!(matches!(
                parsed,
                ParsedArgs::Command { command: "serve", rest, .. } if rest.len() == 2
            ));
            assert_eq!(require_user(Some("Ada")), Ok("Ada"));
            assert_eq!(require_user(None), Err("usuario no encontrado"));
        }
    }
}

pub mod c10 {
    use std::error::Error;
    use std::fmt;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Quantity(u32);

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum QuantityError {
        Zero,
        AboveMaximum { maximum: u32, actual: u32 },
        Overflow,
    }

    // SOLUTION: C10-E01
    impl Quantity {
        pub fn new(value: u32, maximum: u32) -> Result<Self, QuantityError> {
            if value == 0 {
                return Err(QuantityError::Zero);
            }
            if value > maximum {
                return Err(QuantityError::AboveMaximum {
                    maximum,
                    actual: value,
                });
            }
            Ok(Self(value))
        }

        pub fn checked_add(self, other: Self, maximum: u32) -> Result<Self, QuantityError> {
            let value = self.0.checked_add(other.0).ok_or(QuantityError::Overflow)?;
            Self::new(value, maximum)
        }

        pub fn get(self) -> u32 {
            self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub id: u64,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RepositoryError {
        Unavailable,
    }

    // SOLUTION: C10-E02
    pub fn find_in_memory(users: &[User], id: u64) -> Option<&User> {
        users.iter().find(|user| user.id == id)
    }

    pub fn repository_find(
        users: &[User],
        id: u64,
        available: bool,
    ) -> Result<Option<User>, RepositoryError> {
        if !available {
            return Err(RepositoryError::Unavailable);
        }
        Ok(find_in_memory(users, id).cloned())
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Email(String);

    impl Email {
        pub fn new(value: impl Into<String>) -> Self {
            Self(value.into())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum EmailError {
        Missing,
        MissingField { field: String },
    }

    // SOLUTION: C10-E03
    pub fn require_email(email: Option<Email>) -> Result<Email, EmailError> {
        email.ok_or(EmailError::Missing)
    }

    pub fn require_email_for(email: Option<Email>, field: &str) -> Result<Email, EmailError> {
        email.ok_or_else(|| EmailError::MissingField {
            field: field.to_owned(),
        })
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum GetUserError {
        Repository(RepositoryError),
        NotFound { id: u64 },
    }

    impl From<RepositoryError> for GetUserError {
        fn from(source: RepositoryError) -> Self {
            Self::Repository(source)
        }
    }

    // SOLUTION: C10-E04
    pub fn get_user(
        id: u64,
        result: Result<Option<User>, RepositoryError>,
    ) -> Result<User, GetUserError> {
        result?.ok_or(GetUserError::NotFound { id })
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RegisterUserError {
        Repository(RepositoryError),
    }

    impl From<RepositoryError> for RegisterUserError {
        fn from(source: RepositoryError) -> Self {
            Self::Repository(source)
        }
    }

    // SOLUTION: C10-E05
    pub fn register_with_map_err(
        save_result: Result<(), RepositoryError>,
    ) -> Result<(), RegisterUserError> {
        save_result.map_err(RegisterUserError::Repository)?;
        Ok(())
    }

    pub fn register_with_from(
        save_result: Result<(), RepositoryError>,
    ) -> Result<(), RegisterUserError> {
        save_result?;
        Ok(())
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum UsernameError {
        Empty,
        InvalidCharacter { index: usize, character: char },
    }

    impl fmt::Display for UsernameError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Empty => write!(formatter, "el nombre no puede estar vacío"),
                Self::InvalidCharacter { index, character } => {
                    write!(formatter, "carácter '{character}' inválido en {index}")
                }
            }
        }
    }

    impl Error for UsernameError {}

    #[derive(Debug, Eq, PartialEq)]
    pub enum CreateUserError {
        InvalidUsername(UsernameError),
    }

    impl fmt::Display for CreateUserError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::InvalidUsername(_) => write!(formatter, "no se pudo crear el usuario"),
            }
        }
    }

    // SOLUTION: C10-E06
    impl Error for CreateUserError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::InvalidUsername(source) => Some(source),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderStatus {
        Pending,
        Shipped,
        Cancelled,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderError {
        AlreadyShipped,
        AlreadyCancelled,
    }

    // SOLUTION: C10-E07
    pub fn cancel(status: &mut OrderStatus) -> Result<(), OrderError> {
        match status {
            OrderStatus::Pending => {
                *status = OrderStatus::Cancelled;
                Ok(())
            }
            OrderStatus::Shipped => Err(OrderError::AlreadyShipped),
            OrderStatus::Cancelled => Err(OrderError::AlreadyCancelled),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn quantity_preserves_limits_and_arithmetic_failures() {
            let three = Quantity::new(3, 100).unwrap();
            let four = Quantity::new(4, 100).unwrap();
            assert_eq!(three.checked_add(four, 100).map(Quantity::get), Ok(7));
            assert_eq!(Quantity::new(0, 100), Err(QuantityError::Zero));
            assert_eq!(
                Quantity::new(101, 100),
                Err(QuantityError::AboveMaximum {
                    maximum: 100,
                    actual: 101,
                })
            );

            let maximum = Quantity::new(u32::MAX, u32::MAX).unwrap();
            let one = Quantity::new(1, u32::MAX).unwrap();
            assert_eq!(
                maximum.checked_add(one, u32::MAX),
                Err(QuantityError::Overflow)
            );
        }

        #[test]
        fn queries_distinguish_absence_from_failure() {
            let users = [User { id: 7 }];
            assert_eq!(find_in_memory(&users, 7), Some(&users[0]));
            assert_eq!(repository_find(&users, 8, true), Ok(None));
            assert_eq!(
                repository_find(&users, 7, false),
                Err(RepositoryError::Unavailable)
            );
        }

        #[test]
        fn option_can_be_promoted_with_eager_or_lazy_context() {
            let email = Email::new("ada@example.com");
            assert_eq!(require_email(Some(email.clone())), Ok(email));
            assert_eq!(require_email(None), Err(EmailError::Missing));
            assert_eq!(
                require_email_for(None, "contact_email"),
                Err(EmailError::MissingField {
                    field: String::from("contact_email"),
                })
            );
        }

        #[test]
        fn application_errors_preserve_absence_and_causes() {
            assert_eq!(get_user(7, Ok(None)), Err(GetUserError::NotFound { id: 7 }));
            assert_eq!(
                get_user(7, Err(RepositoryError::Unavailable)),
                Err(GetUserError::Repository(RepositoryError::Unavailable))
            );
            assert_eq!(
                register_with_map_err(Err(RepositoryError::Unavailable)),
                Err(RegisterUserError::Repository(RepositoryError::Unavailable))
            );
            assert_eq!(
                register_with_from(Err(RepositoryError::Unavailable)),
                Err(RegisterUserError::Repository(RepositoryError::Unavailable))
            );
        }

        #[test]
        fn error_trait_and_domain_transition_expose_the_right_contract() {
            let error = CreateUserError::InvalidUsername(UsernameError::Empty);
            assert_eq!(error.to_string(), "no se pudo crear el usuario");
            assert_eq!(
                error.source().unwrap().to_string(),
                "el nombre no puede estar vacío"
            );

            let mut pending = OrderStatus::Pending;
            assert_eq!(cancel(&mut pending), Ok(()));
            assert_eq!(pending, OrderStatus::Cancelled);
            assert_eq!(cancel(&mut pending), Err(OrderError::AlreadyCancelled));

            let mut shipped = OrderStatus::Shipped;
            assert_eq!(cancel(&mut shipped), Err(OrderError::AlreadyShipped));
        }
    }
}
