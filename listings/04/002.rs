struct Tracer(&'static str);

impl Drop for Tracer {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _outer = Tracer("outer");

    {
        let _first = Tracer("first");
        let _second = Tracer("second");
        println!("inside");
    }

    println!("outside");
}
