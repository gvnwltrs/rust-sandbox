use std::fs;
use std::fs::File;
use std::io::{self, Read};
use heapless::String;

#[derive(Debug)]
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

    // Returing a reference to byte.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    // Returning a reference to the string that is owned by FileBuf.
    // The caller get's to borrow the string (read-only).
    pub fn as_str(&self) -> io::Result<&str> {
        std::str::from_utf8(self.as_bytes())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))
    }

    pub fn tokenize<'a>(&self, s: &'a str) -> impl Iterator<Item = &'a str> {
        s.split('\n')
        // or s.lines()
    }

    // `map_err` will iterate over the result and identify if there is a non-UTF-8 error.
    // In this case, where are returning a copy of string to the caller
    // so that the caller takes ownership of the string.
    pub fn extract_all(&self) -> io::Result<String<N>> {
        // Check that the buffer contains valid UTF-8 data.
        let text = std::str::from_utf8(&self.buf[..self.len]) // from 0 to len
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        // Create a heapless String and copy the text into it.
        let mut out = String::<N>::new();
        // Safe to unwrap here because we have already checked the size.
        for line in text.lines() {
            out.push_str(line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "string too large"))?;
            out.push('\n').map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "string too large"))?;
        }
        Ok(out)
    }

    // Returing a Result<T, E> with reference to the vector of a strings (tokens) to the caller. 
    // FileBuf owns, caller borrows (read-only).
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

    // Returning Result<T, E> with a reference to a vector of strings to a caller. 
    // FileBuf still owns the vector of strings, and the caller borrows.
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

    // Returning Result<T, E> with a reference to a vector of strings to a caller. 
    // FileBuf still owns the vector of strings, and the caller borrows.
    pub fn extract_infos<'a>(&'a self) -> io::Result<Vec<&'a str>> {
        let text = self.as_str()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let mut infos = Vec::<&str>::new();
        for line in text.lines() {
            if line.starts_with("INFO") {
                infos.push(line);
            }
        }
        Ok(infos)
    }

    pub fn export_to_file(&self, logs: &Vec<&str>, path: &str) -> Result<(), std::io::Error> {
        let data = logs.join("\n");
        let result = match fs::write(path, data) {
            Ok(..) => println!("Wrote to file successfully."),
            Err(e) => {
                println!("Failed to write to file: {}", e);
            },
        };
        println!("Exported to file: {}", path);
        Ok(result)
    }

    pub fn print_all<T: AsRef<str> + std::fmt::Debug>(&self, tag: &str, log: &T) { 
        match tag {
            "FULL_LOG" => { 
                println!("================== {} ==================", tag.to_uppercase());
            }
            "ALL" => { 
                println!("================== {} ==================", tag.to_uppercase()) 
            }
            _ => {
                println!("<Tag not recognized. TAGS: FULL_LOG, ALL>");
                println!("================== LOG ==================");
                return; // prevents breaking non non-full log prints
            }
        }
        self.tokenize(log.as_ref()).for_each(|line| println!("{}", line));
    }

    pub fn print_log<T: std::fmt::Debug>(&self, tag: &str, log: &T) { 
        match tag {
            "INFOS" => { 
                println!("================== {} ==================", tag.to_uppercase()) 
            }
            "WARNINGS" => { 
                println!("================== {} ==================", tag.to_uppercase()) 
            }
            "ERRORS" => { 
                println!("================== {} ==================", tag.to_uppercase()) 
            }
            _ => {
                println!("<Tag not recognized. TAGS: INFOS, WARNINGS, ERRORS>");
                println!("================== LOG ==================");
            }
        }
        println!("{:?}: {:#?}", tag.to_uppercase(), log);
    }

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_extract_errors() {
        let mut file_buf = FileBuf::<2048>::new();
        match file_buf.read_to_buf("logs.txt") {
            Ok(_) => (),
            Err(e) => panic!("Error reading to buffer: {}", e),
        }
        let errors = file_buf.extract_errors();
        // assert_eq!(errors[0], "ERROR 14:33:45 Failed to connect to the database.");
        match errors {
            Ok(_) => assert_eq!(errors.unwrap()[0], "ERROR 14:33:45 Failed to connect to the database."),
            Err(e) => panic!("Error extracting errors: {}", e),
        }
    }

    #[test]
    fn test_extract_warnings() {
        let mut file_buf = FileBuf::<2048>::new();
        match file_buf.read_to_buf("logs.txt") {
            Ok(_) => (),
            Err(e) => panic!("Error reading to buffer: {}", e),
        }
        let warnings = file_buf.extract_warnings();
        match warnings {
            Ok(_) => assert_eq!(warnings.unwrap()[2], "WARNING 14:42:30 Low disk space on the backup drive."),
            Err(e) => panic!("Error extracting warnings: {}", e),
        }
    }

    #[test]
    fn test_extract_infos() {
        let mut file_buf = FileBuf::<2048>::new();
        match file_buf.read_to_buf("logs.txt") {
            Ok(_) => (),
            Err(e) => panic!("Error reading to buffer: {}", e),
        }
        let infos = file_buf.extract_infos();
        match infos {
            Ok(_) => assert_eq!(infos.unwrap()[1], "INFO 14:35:20 User logged in successfully."),
            Err(e) => panic!("Error extracting infos: {}", e),
        }
    }
}

