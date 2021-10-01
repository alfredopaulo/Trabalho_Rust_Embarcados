fn main() {
    let a:u16 = 0x7ff;
    let result:u16;
    
   result = !a;
   println!("(!a) => 0x{:x} ",result);
}
