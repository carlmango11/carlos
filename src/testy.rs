static HELLO: &[u8] = b"DERP";

fn main() {
    for (i, byte) in HELLO.iter().enumerate() {
        println!("Byte {}: {}", i, *byte as char);
    }
}
