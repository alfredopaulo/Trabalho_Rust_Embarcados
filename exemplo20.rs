fn main() {
    let x: u8 = 1;
    println!("Operador de deslocamento - Bits à Esquerda\nValor de x: {:0>8b}", x);
    println!("x << 0 = {:0>8b}", x << 0);//Não deslocado
    println!("x << 1 = {:0>8b}", x << 1);
    println!("x << 2 = {:0>8b}", x << 2);
    println!("x << 3 = {:0>8b}", x << 3);
    println!("x << 4 = {:0>8b}", x << 4);
    println!("x << 5 = {:0>8b}", x << 5);
    println!("x << 6 = {:0>8b}", x << 6);
    println!("x << 7 = {:0>8b}", x << 7);
}
