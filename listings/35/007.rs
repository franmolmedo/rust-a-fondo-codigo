use std::mem::size_of_val;

fn main() {
    let tiny = async {};

    let with_buffer = async {
        let buffer = [7_u8; 1024];
        std::future::ready(()).await;
        std::hint::black_box(buffer)[0]
    };

    println!("empty async block: {} bytes", size_of_val(&tiny));
    println!("buffer used after await: {} bytes", size_of_val(&with_buffer));
}
