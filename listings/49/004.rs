macro_rules! make_newtypes {
    ($( $name:ident($inner:ty) ),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            struct $name($inner);
        )+
    };
}

make_newtypes!(UserId(u64), OrderId(u64),);
assert_eq!(UserId(7).0, 7);
assert_eq!(OrderId(9).0, 9);
