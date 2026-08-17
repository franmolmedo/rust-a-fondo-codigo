fn main() {
    let future = async {
        println!("cuerpo ejecutado");
        42
    };

    // Sin executor que lo sondee, el cuerpo nunca corre:
    drop(future);
    println!("fin del programa");
}
