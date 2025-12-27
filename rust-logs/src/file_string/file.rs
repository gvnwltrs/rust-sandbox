use std::fs::File;
use std::io::{self, Read};
use heapless::String;

// TODO: add struct with length and buf
pub struct FileBuf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> FileBuf<N> {
    pub fn new() -> Self {
        Self { buf: [0u8; N], len: 0 }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn read_to_buf(&mut self, path: &str) -> io::Result<()> {
        self.clear();
        let mut file = File::open(path)?;

        loop {
            if self.len >= N {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "file too large"));
            }
            let n = file.read(&mut self.buf[self.len..])?;
            if n == 0 { break; }
            self.len += n;
        }
        Ok(())
    }

    // map_err will iterate over the result and identify if there is a non-UTF-8 error
    pub fn extract_all(&self) -> io::Result<String<N>> {
        let text = std::str::from_utf8(&self.buf[..self.len]) // from 0 to len
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let mut out = String::<N>::new();
        out.push_str(text)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "file too long"))?; // safe: we just checked the size
        Ok(out)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.as_bytes())
    }

    pub fn extract_errors<'a>(&'a self) -> io::Result<Vec<&'a str>> {
        let text = self.as_str()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let mut errors = Vec::<&str>::new();
        for line in text.lines() {
            if line.starts_with("ERROR") {
                errors.push(line);
            }
        }
        Ok(errors)
    }

    pub fn extract_warnings<'a>(&'a self) -> io::Result<Vec<&'a str>> {
        let text = self.as_str()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let mut warnings = Vec::<&str>::new();
        for line in text.lines() {
            if line.starts_with("WARNING") {
                warnings.push(line);
            }
        }
        Ok(warnings)
    }
}