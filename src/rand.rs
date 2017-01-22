use std::io;
use ring::rand;

pub fn fill(buffer: &mut [u8]) -> io::Result<()> {
    rand::SystemRandom::new()
        .fill(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Random generation failed"))
}
