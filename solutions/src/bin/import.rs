use course_solutions::organization::c27::run_import;

fn main() {
    let value = std::env::args().nth(1).unwrap_or_default();
    println!("{}", run_import(&value));
}
