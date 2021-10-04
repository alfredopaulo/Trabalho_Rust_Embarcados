fn main(){
    let m1:u16 = 0xE3;
    let m2:u16 = 0x14;
    let mut b:u16 = 0xAF;

    println!("(m1) » 0x{:x} ", m1);
    println!("(m2) » 0x{:x} ", m2);
    println!("(b) » 0x{:x} ", b);
    println!("b = ((b & m1) | m2) ");
    b = (b & m1) | m2;
    println!("b = 0x{:x} ", b);
}
