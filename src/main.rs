fn main() {
    let bootrom = include_bytes!("../dmg0.bin");
    println!("{:x?}", bootrom);
}
