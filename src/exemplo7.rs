fn main(){
    let b:u16 = 0xa726;
    let result:u16;

    result = !b;
    println!("(b) => 0x{:x}", b);
    println!("(!b) => 0x{:x}", result);
}