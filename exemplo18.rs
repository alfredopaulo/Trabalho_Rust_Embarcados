fn main(){
    let mut b:u16 = 0xAF;
    let m1:u16 = 0xFC;
    let m2:u16 = 0x02;

    println!("(b) » 0x{:x} ", b);
    println!("(M1) » 0x{:x} ", m1);
    println!("(M2) » 0x{:x} ", m2);
    println!("b = ((b & M1) | M2) ");
    b = (b & m1) | m2;
    println!("b = 0x{:x} ", b);
}
