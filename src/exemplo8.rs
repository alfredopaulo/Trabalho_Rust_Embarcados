fn main(){
    let a:u16 = 0x6db7;
    let b:u16 = 0xa726;
    let result:u16;

    result = a & b;
    println!("(a) => 0x{:x} ", a);
    println!("(b) => 0x{:x} ", b);
    println!("(a & b) => 0x{:x} ", result);

}