
/* Project Dependencies */
use crate::rca::{ State, Mode, TASK_BUFFER };

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
 * with thercaorse of the system. This could be graphical display, system device interaction,
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
    pub config: ConfigData,         // (0) Init state: details for initalization & configuration of system behavior
    pub read_io: ReadData,        // (2) Running state: import data (e.g. file system or API call) 
    pub write_io: WriteData,       // (2) Running state: export data (e.g. file system or API call)
    pub display_io: DisplayData,     // (2) Running state: utilizing system terminal output or display drivers
    pub perf: PerfData,           // (2) Running state: system information details 
    pub logs: LogData,    // (2, 3, 4, 5) Running, Failure, Degraded, Shutdown state: Logs for any event during running state  
    pub task_buffer: usize,
    pub task_desc: String,
    pub mode: Mode,
    pub state: State,                   // System state
}

/* Status: FREEZE */
impl Default for Data {
    fn default() -> Self {
        Self {
            config: ConfigData::None,
            read_io: ReadData::None,
            write_io: WriteData::None,
            display_io: DisplayData::default(),
            perf: PerfData::None,
            logs: LogData::None,
            task_buffer: TASK_BUFFER,
            task_desc: Default::default(),
            mode: Mode::Debug,
            state: State::Init,
        }
    }

    /* Micro-kernel space (Loop Engine privelage only):
    * Apply returned outputs to ctx.
    * This is the missing link that makes "returns" actually do something.
    */
}

/*******************************************************************************
 * (2) Add custom data models here 
******************************************************************************/

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DisplayData {
    pub title: String,
    pub body: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SystemData {}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum WriteData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum ReadData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum PerfData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum LogData {
    None,
    Session {
        entry: String,
        date: String,
    }
} 
