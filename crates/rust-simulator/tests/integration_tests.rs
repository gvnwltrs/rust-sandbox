// extern crate rust_simulator;
use rust_simulator::functional_sim::*;
use timebomb::timeout_ms;

#[test]
fn test_main() {
    timeout_ms(|| {
        run_simulation();
    }, 30_000);
}