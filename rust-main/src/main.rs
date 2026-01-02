use core::ops::ControlFlow;

mod repeat_until;
use repeat_until::repeat_until::repeat_until;

mod guess_number_v1;
use guess_number_v1::guess_number_v1::run_engine;

mod guess_number_v2;
use guess_number_v2::guess_number_v2::Game;

fn count_to_5() {
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

fn main() {
    println!("Rust Main Starting...");
    
    // count_to_5();
    // run_engine();
    let mut new_game = Game::setup(42, 5);
    new_game.start();

}
