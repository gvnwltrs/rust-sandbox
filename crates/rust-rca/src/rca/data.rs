
/*******************************************************************************
 * Data plane
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

/* Status: FREEZE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct DataPlane {
    pub config: ConfigData,       // Init state: details for initalization & configuration of system behavior
    pub read_io: ReadData,        // Running state: import data (e.g. file system or API call) 
    pub write_io: WriteData,      // Running state: export data (e.g. file system or API call)
    pub perf: PerfData,           // Running state: system information details 
    pub logs: LogData,            // Running, Failure, Degraded, Shutdown state: Logs for any event during running state  
    pub cells: CellInfo,          // Running state: Contains cell count and possibly other metadata for cells.
    pub activity: ActivityInfo,   // Running state: Contains current task details.
    pub display: DisplayInfo,     // Running state: utilizing system terminal output or display drivers
}

/*******************************************************************************
 * Apex data models 
******************************************************************************/

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum ReadData {
    None,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum WriteData {
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

#[derive(Debug, Clone, PartialEq)]
pub struct CellInfo {
    pub count: usize,
} 

impl Default for CellInfo {
    fn default() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActivityInfo {
    pub description: String,
} 

impl Default for ActivityInfo {
    fn default() -> Self {
        Self { description: Default::default() } 
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DisplayInfo {
    pub title: String,
    pub body: String,
    pub status: String,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SystemData {
    pub description: String,
}

/*******************************************************************************
 * Add custom data models here 
******************************************************************************/
