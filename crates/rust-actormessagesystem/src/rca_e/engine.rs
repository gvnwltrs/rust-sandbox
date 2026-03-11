use std::io::Error;
use std::io::ErrorKind;

#[allow(unused)]
use crate::rca_e::{ 
    Data, 
    SystemData, 
    DisplayModel, 
    ActorId, 
    Message,
    MessageKind, 
    ProgramThread, 
    TaskType, 
    Cell, 
    CellData, 
    State 
};

/*******************************************************************************
 * (1) Default 
******************************************************************************/

#[derive(Debug)]
pub struct Engine {
    pub ctx: Data,
    pub sys: SystemData,
}

pub trait PrimaryRunner {
    fn give_default() -> Self;
    fn try_run_engine(&mut self) -> Result<(), Error>;
    fn prepare_cycle(&mut self) -> Result<(), Error>;
    fn run_thread(&mut self) -> Result<(), Error>;
    fn regulate_results(&mut self) -> Result<(), Error>;
    fn render_cycle_output(&self) -> Result<String, Error>;
    fn bump_actor(&mut self, actor_id: ActorId) -> Result<(), Error>; 
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

impl PrimaryRunner for Engine {
     fn give_default() -> Self {
        Self {
            ctx: Data::default(),
            sys: SystemData::default(),
        }
    }

    fn try_run_engine(&mut self) -> Result<(), Error> {
        self.ctx.state = State::Running;

        while self.ctx.state == State::Running {
            if self.sys.completed {
                self.ctx.state = State::Shutdown;
                break;
            }

            if self.sys.queue.is_empty() {
                self.ctx.state = State::Idle;
                break;
            }

            self.prepare_cycle()?;
            self.run_thread()?;
            self.regulate_results()?;

            let output = self.render_cycle_output()?;
            println!("{:#?}\n", output);
        }

        Ok(())
    }

    fn prepare_cycle(&mut self) -> Result<(), Error> {
        self.ctx.write_io = None;
        self.ctx.display_io = None;
        self.ctx.task_desc = None;
        self.ctx.cur_cell_id = Some(0);
        self.ctx.prev_cell_id = Some(0);

        let Some(msg) = self.sys.queue.pop_front() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: queue was unexpectedly empty",
            ));
        };

        self.sys.current_msg = Some(msg);
        Ok(())
    }

    fn run_thread(&mut self) -> Result<(), Error> {
        let msg = self.sys.current_msg.clone().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Engine: missing current message")
        })?;

        let mut thread = ProgramThread::build_tasks(
            Some(0),
            Some([
                Cell { id: 0, task: TaskType::LoadNextMessage },
                Cell { id: 1, task: TaskType::DispatchActor },
                Cell { id: 2, task: TaskType::RenderEvent },
            ]),
            Some(CellData::Message(msg)),
        );

        while !thread.is_finished() {
            thread.step(&mut self.ctx)?;
        }

        let final_handoff = thread.take_handoff();

        match final_handoff {
            CellData::String(rendered) => {
                self.sys.last_output = Some(rendered.clone());
                self.ctx.write_io = Some(rendered.clone());
                self.ctx.display_io = Some(DisplayModel {
                    title: "RCA-E Actor System".into(),
                    body: rendered,
                    status: "OK".into(),
                });
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Engine: final handoff was not render output",
                ));
            }
        }

        Ok(())
    }

    fn regulate_results(&mut self) -> Result<(), Error> {
        let msg = self.sys.current_msg.clone().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Engine: missing current message in regulate_results")
        })?;

        self.bump_actor(msg.to)?;

        match (msg.to, msg.kind) {
            (ActorId::A, MessageKind::Start) => {
                self.sys.queue.push_back(Message {
                    from: ActorId::A,
                    to: ActorId::B,
                    kind: MessageKind::Ping,
                });
            }
            (ActorId::B, MessageKind::Ping) => {
                self.sys.queue.push_back(Message {
                    from: ActorId::B,
                    to: ActorId::C,
                    kind: MessageKind::Ping,
                });
            }
            (ActorId::C, MessageKind::Ping) => {
                self.sys.queue.push_back(Message {
                    from: ActorId::C,
                    to: ActorId::A,
                    kind: MessageKind::Done,
                });
            }
            (ActorId::A, MessageKind::Done) => {
                self.sys.completed = true;
                self.ctx.state = State::Shutdown;
            }
            _ => {
                self.ctx.state = State::Failure;
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Engine: unsupported transition in regulate_results",
                ));
            }
        }

        Ok(())
    }

    fn bump_actor(&mut self, actor_id: ActorId) -> Result<(), Error> {
        let Some(actor) = self.sys.actors.iter_mut().find(|a| a.id == actor_id) else {
            return Err(Error::new(ErrorKind::InvalidData, "Engine: actor not found"));
        };

        actor.handled_count += 1;
        Ok(())
    }

    fn render_cycle_output(&self) -> Result<String, Error> {
        let Some(display) = self.ctx.display_io.as_ref() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: display_io was not populated",
            ));
        };

        Ok(display.body.clone())
    }

}
