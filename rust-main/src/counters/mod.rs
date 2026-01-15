use crate::repeat_until::repeat_until;
use core::ops::ControlFlow;
#[allow(unused)]
use std::io::{stdin, BufRead, Cursor};

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

pub fn count_lines(input: &String) -> usize {
    input.lines().count()
}

#[test]
fn test_count_lines_count_is_correct() {
    let input = Cursor::new("line 1\nline 2\n");
    let lines = count_lines(&input.into_inner().to_string());
    assert_eq!(lines, 2, "wrong line count");
}