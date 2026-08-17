mod tokens {
    mod private {
        pub trait Sealed {}
    }

    pub struct Word(pub String);

    impl private::Sealed for Word {}

    pub trait Token: private::Sealed {
        fn text(&self) -> &str;
    }

    impl Token for Word {
        fn text(&self) -> &str {
            &self.0
        }
    }
}

use tokens::{Token, Word};

fn main() {
    assert_eq!(Word(String::from("rust")).text(), "rust");
}
