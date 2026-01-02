use core::ops::ControlFlow;

// A zero-allocation iterator engine.
// Each call to `next()` advances the state machine. 
pub fn repeat_until<F, T>(mut step: F) -> impl Iterator<Item = T>
where
    F: FnMut() -> ControlFlow<(), T>,
{
    std::iter::from_fn(move || match step() {
        ControlFlow::Continue(item) => Some(item),
        ControlFlow::Break(()) => None,
    })
}