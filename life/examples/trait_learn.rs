use core::fmt;
use std::io::Write;

struct BufBuilder {
    buf: Vec<u8>,
}
impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hello-debug:{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let mut buf = BufBuilder::new();
    let v = b"hello world!";
    buf.write_all(v).unwrap();
    println!("{:?}", buf);
}
