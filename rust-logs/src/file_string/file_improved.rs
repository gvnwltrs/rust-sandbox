// Improved version with better embedded practices

use std::fs::File;
use std::io::{self, Read};
use heapless::String;

/// Alternative: For embedded, use Read trait (works with no_std readers)
pub fn read_to_stack_string_from_reader<const N: usize, R: Read>(
    reader: &mut R,
) -> io::Result<String<N>> {
    let mut buf = [0u8; N];
    let mut len = 0;

    loop {
        if len == N {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "file too large"));
        }
        let n = reader.read(&mut buf[len..])?;
        if n == 0 { break; }
        len += n;
    }

    let text = core::str::from_utf8(&buf[..len])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
    
    let mut out = String::<N>::new();
    out.push_str(text).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "string too long"))?;
    Ok(out)
}

// For true embedded (no_std), you'd use something like:
// use embedded_io::Read;
// pub fn read_to_stack_string_embedded<const N: usize, R: embedded_io::Read>(
//     reader: &mut R,
// ) -> Result<String<N>, R::Error> {
//     // Similar logic but with embedded_io traits
// }

