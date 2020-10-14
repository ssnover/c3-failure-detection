use std::fs::File;
use std::io::prelude::*;

pub enum LogEvent {
    Join { joined_id: u8 },
    Failure { failed_id: u8, reporting_id: u8 },
}

pub struct Logger {
    log_events: Vec<(u32, LogEvent)>,
    log_file: File,
}

impl Logger {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Logger> {
        Ok(Logger {
            log_events: vec![],
            log_file: File::create(path)?,
        })
    }

    pub fn log_join_event(&mut self, timestep: u32, joined_node_id: u8) {
        self.log_events.push((
            timestep,
            LogEvent::Join {
                joined_id: joined_node_id,
            },
        ));
        self.log_file
            .write_all(format!("{}: Node {} joined\r\n", timestep, joined_node_id).as_bytes())
            .unwrap();
    }

    pub fn log_failure_event(&mut self, timestep: u32, failed_node_id: u8, reporting_node_id: u8) {
        self.log_events.push((
            timestep,
            LogEvent::Failure {
                failed_id: failed_node_id,
                reporting_id: reporting_node_id,
            },
        ));
        self.log_file
            .write_all(
                format!(
                    "{}: Node {} marked node {} as failed",
                    timestep, failed_node_id, reporting_node_id
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub fn count_join_events(&self) -> usize {
        self.log_events
            .iter()
            .filter(|&event| match event.1 {
                LogEvent::Join { joined_id: _ } => true,
                _ => false,
            })
            .count()
    }

    pub fn count_failure_events(&self) -> usize {
        self.log_events
            .iter()
            .filter(|&event| match event.1 {
                LogEvent::Failure {
                    failed_id: _,
                    reporting_id: _,
                } => true,
                _ => false,
            })
            .count()
    }
}
