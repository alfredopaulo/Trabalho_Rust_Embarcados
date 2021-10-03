//Walter Jonas
//Operadores de Deslocamento - Deslocando Bits a Direita
//Exemplo 22

fn main() {
    let a: u16 = 0b0110110110110111;
    let b: u16 = a >> 6;
    println!("Operador de deslocamento - Bits a direita\nValor de a: {:0>16b}", a);
    println!("a >> 6 = {:0>16b}", b);
    println!("a >> 6 em Hexadecimal = 0x{:x}", b);
}
