use std::io;
use ring::rand::{SecureRandom, SystemRandom};

pub fn fill(buffer: &mut [u8]) -> io::Result<()> {
    SystemRandom::new()
        .fill(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Random generation failed"))
}
