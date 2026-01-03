use core::ops::ControlFlow;
use crate::repeat_until::repeat_until::repeat_until;

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