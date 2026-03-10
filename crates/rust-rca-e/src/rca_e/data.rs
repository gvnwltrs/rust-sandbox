use std::io::Error;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_e::{ State, TaskOutput, CellData, TASK_BUFFER };

/*******************************************************************************
 * (1) Data
 * 
 * Establish data endpoints. 
 * Establish & confirm complete data. 
 *
******************************************************************************/

/* Apex Data:
 * The primary goal of this data is to represent the state of the overall system. 
 * The data here is only the essential data for which to affect the system with,
 * or to provide the status & performance of the system. "Affecting the system" 
 * means to produce data such that it produces a particular desired result in line
 * with the purporse of the system. This could be graphical display, system device interaction,
 * pure logging or data etc. By constraining the overall apex data to just these,
 * we can essentially affect any part of the system while maintaining clarity for 
 * what we are trying to do strictly within the constraints or scope of the design. 
 */

/* Status: 
    - fields: FREEZE 
    - values: MUTABLE
*/
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct Data {
    pub config: Option<String>,         // (0) Init state: details for initalization & configuration of system behavior
    pub read_io: Option<String>,        // (2) Running state: import data (e.g. file system or API call) 
    pub write_io: Option<String>,       // (2) Running state: export data (e.g. file system or API call)
    pub display_io: Option<DisplayModel>,     // (2) Running state: utilizing system terminal output or display drivers
    pub perf: Option<String>,           // (2) Running state: system information details 
    pub logs: Option<[String; 100]>,    // (2, 3, 4, 5) Running, Failure, Degraded, Shutdown state: Logs for any event during running state  
    pub cur_cell_id: Option<usize>,         // Introspection into current activity
    pub prev_cell_id: Option<usize>,            // Access index: Current cell can access previous cell generated data
    pub debug_mode: Option<String>,
    pub task_buffer: usize,
    pub task_desc: Option<String>,
    pub state: State,                   // System state
}

/* Status: FREEZE */
impl Data {
    pub fn give_system_init() -> Self {
        Self {
            read_io: None,
            write_io: None,
            display_io: None,
            config: None,
            perf: None,
            logs: None,
            cur_cell_id: Some(0),
            prev_cell_id: Some(0),
            debug_mode: Some(String::from("Default")),
            task_buffer: TASK_BUFFER,
            task_desc: None,
            state: State::Init,
        }
    }

    /* Micro-kernel space (Loop Engine privelage only):
    * Apply returned outputs to ctx.
    * This is the missing link that makes "returns" actually do something.
    */

    /* Status: MUTABLE */
    pub fn mutate_state(&mut self, _in: (CellData, TaskOutput)) -> Result<Option<CellData>, Error> {
        match _in {

            ( CellData::DisplayData(data), TaskOutput::MutateDisplayIO ) => { 
                self.display_io = Some(data); 
                Ok(None) 
            }

            ( CellData::String(data), TaskOutput::MutatePerf )  => { 
                self.perf = Some(data); 
                Ok(None) 
            }

            ( any, TaskOutput::NextCell ) => { 

                Ok(Some(any)) 
            }

            _ => Ok(None)
        }
    }
}


/*******************************************************************************
 * (2) Add custom data models here 
******************************************************************************/

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DisplayModel {
    pub title: String,
    pub body: String,
    pub status: String,
}

// NOTE: Idea for future domain implementatoins:
// - MCU projects:
//  - Data contains GPIO, I2C, SPI, UART, Registers, Memory Addresses, etc.