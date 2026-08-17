struct Samples<const N: usize>([f32; N]);

fn compare<const N: usize>(_left: &Samples<N>, _right: &Samples<N>) {}

fn main() {
    let short = Samples([0.0; 8]);
    let long = Samples([0.0; 16]);
    compare(&short, &long);
    // se esperaba Samples<8>, se encontró Samples<16>
}
