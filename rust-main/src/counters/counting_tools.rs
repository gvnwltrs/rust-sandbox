use crate::repeat_until::repeat_until;
use core::ops::ControlFlow;
use std::io;
#[allow(unused)]
use std::io::{stdin, BufReader, Cursor};

pub fn count_to_5() {
    let mut count = 0;
    repeat_until(|| {
        if count >= 5 {
            ControlFlow::Break(())
        } else {
            count += 1;
            println!("Count: {}", count);
            ControlFlow::Continue(())
        }
    })
    .for_each(drop)
}

pub fn count_lines(input: &str) -> io::Result<usize> {
    Ok(input.lines().count())
}   

#[cfg(test)]
mod counter_tests {
    use super::*;

    #[test]
    fn test_count_lines_count_is_correct() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(&input.into_inner().to_string());
        assert!(lines.is_err() != true);
    }

}