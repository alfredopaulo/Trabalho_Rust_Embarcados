fn main(){
    let a:u16 = 0x6DB7;
    let b:u16;
    let c:u16;
    let mask:u16 = 0x4;

    b = a ^ mask;
    println!("(a) » 0x{:x} ", a);
    println!("(mask) » 0x{:x} ", mask);
    println!("b = a & mask ");
    println!("b = 0x{:x} ", b);

    println!("");

    c = b ^ mask;
    println!("(b) » 0x{:x} ", b);
    println!("(mask) » 0x{:x} ", mask);
    println!("c = b & mask ");
    println!("c = 0x{:x} ", c);
}
