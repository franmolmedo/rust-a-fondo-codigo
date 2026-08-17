let names = [String::from("Ada"), String::from("Grace")];
let &[ref first, ref second] = &names;

assert_eq!((first.as_str(), second.as_str()), ("Ada", "Grace"));
