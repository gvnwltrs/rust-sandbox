use std::fs::File;
use std::io::{self, Read};
use heapless::String;

pub fn read_to_stack_string<const N: usize>(path: &str) -> io::Result<String<N>> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; N];
    let mut len = 0;

    loop {
        if len == N {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "file too large"));
        }
        let n = file.read(&mut buf[len..])?;
        if n == 0 { break; }
        len += n;
    }

    let text = std::str::from_utf8(&buf[..len])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;

    let mut out = String::<N>::new();
    out.push_str(text).unwrap(); // safe: we just checked the size
    Ok(out)
}
