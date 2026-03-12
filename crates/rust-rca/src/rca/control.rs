

/*******************************************************************************
 * Control plane 
******************************************************************************/

#[derive(Debug, PartialEq)]
pub struct ControlPlane {
    pub state: State,
    pub mode: Mode,
    pub event: Event,
}

impl Default for ControlPlane {
    fn default() -> Self {
        Self {
            state: State::Init,
            mode: Mode::Debug,
            event: Event::None,
        }
    }
}

/*******************************************************************************
 * (1) States 
******************************************************************************/

/* Status: FREEZE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum State {
    Init,
    Idle,
    Running,
    Halt,
    Failure,
    Shutdown,
}

/*******************************************************************************
 * (2) Modes 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Mode {
    None,
    Debug,
}

/*******************************************************************************
 * (3) Events 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Event {
    None,
}
