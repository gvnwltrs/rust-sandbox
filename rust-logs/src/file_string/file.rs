use std::fs::File;
use std::io::{self, Read};
use heapless::String;

<<<<<<< HEAD
pub fn read_to_stack_string<const N: usize>(path: &str) -> io::Result<String<N>> {
=======
// TODO: add struct with length and buf

// TODO: add read to buf here and wrap in struct with length
pub fn read_to_buf<const N: usize>(path: &str) -> io::Result<[u8; N]> {
>>>>>>> origin/lenovo
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

<<<<<<< HEAD
    let text = std::str::from_utf8(&buf[..len])
=======
    Ok(buf)
}

// TODO: add extract string from buf here
pub fn extract_string_from_buf<const N: usize>(buf: [u8; N], len: usize) -> io::Result<String<N>> {
    let text = std::str::from_utf8(&buf[..len]) // from 0 to len
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
    let mut out = String::<N>::new();
    out.push_str(text).unwrap(); // safe: we just checked the size
    Ok(out)
}

// pub fn extract_errors_from_buf<const N: usize>(buf: [u8; N], len: usize) -> io::Result<Vec<String<N>>> {
//     let text = std::str::from_utf8(&buf[..len]) // from 0 to len
//         .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
//     let mut errors = Vec::<String<N>>::new();
//     for line in text.lines() {
//         if line.starts_with("ERROR") {
//             errors.push(String::<N>::from(line));
//         }
//     }
//         Ok(errors)
// }

pub fn read_to_stack_string<const N: usize>(path: &str) -> io::Result<String<N>> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; N]; // reading byte characters into buffer of size N
    let mut len = 0;

    // Check lenght of file and read into buffer
    loop {
        if len == N {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "file too large"));
        }
        let n = file.read(&mut buf[len..])?;
        if n == 0 { break; }
        len += n;
    }

    // Converting bytes characters to string
    let text = std::str::from_utf8(&buf[..len]) // from 0 to len
>>>>>>> origin/lenovo
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;

    let mut out = String::<N>::new();
    out.push_str(text).unwrap(); // safe: we just checked the size
    Ok(out)
}
